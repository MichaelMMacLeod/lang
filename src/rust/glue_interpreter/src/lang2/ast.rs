use std::collections::{HashMap, HashSet};

use crate::{
    index::ZeroPlus,
    lang0::{
        self,
        expr::{ConstantExpr, Var},
        stmt::Label,
    },
    lang1::stmt::{Index, IndexElement},
    storage::StorageKey,
};
use lang0::ast::Ast as Ast0;
use lang0::expr::Expr as Expr0;
use lang0::stmt::Stmt as Stmt0;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LoopEnd {
    len_minus: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Ast {
    Sym(StorageKey),
    Copy(Index),
    Build(Vec<AstLoopable>),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum AstLoopable {
    Ast(Ast),
    ForLoop {
        var: usize,
        start: usize,
        end: LoopEnd,
        prior: Vec<ConstantExpr>,
        body: Box<AstLoopable>,
    },
}

impl Ast {
    pub fn sym(key: StorageKey) -> Self {
        Self::Sym(key)
    }

    pub fn copy(elements: Vec<IndexElement>) -> Self {
        Self::Copy(Index::new(elements))
    }

    pub fn build(elements: Vec<AstLoopable>) -> Self {
        Self::Build(elements)
    }
}

impl AstLoopable {
    pub fn sym(key: StorageKey) -> Self {
        Self::Ast(Ast::sym(key))
    }

    pub fn copy(elements: Vec<IndexElement>) -> Self {
        Self::Ast(Ast::copy(elements))
    }

    pub fn build(elements: Vec<AstLoopable>) -> Self {
        Self::Ast(Ast::build(elements))
    }

    pub fn for_loop(
        var: lang0::expr::Var,
        start: usize,
        end_at_len_minus: usize,
        prior: Vec<ConstantExpr>,
        body: AstLoopable,
    ) -> Self {
        Self::ForLoop {
            var: var.0,
            start,
            end: LoopEnd {
                len_minus: end_at_len_minus,
            },
            body: Box::new(body),
            prior,
        }
    }
}

impl Ast {
    fn compile(self) -> Ast0 {
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        struct SrcVar(usize);

        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        struct GenVar(usize);

        let mut varmap: HashMap<SrcVar, GenVar> = HashMap::new();
        let mut vars: HashSet<GenVar> = HashSet::new();

        fn new_gen_var(vars: &mut HashSet<GenVar>) -> usize {
            let mut var = vars.len();
            loop {
                if !vars.contains(&GenVar(var)) {
                    vars.insert(GenVar(var));
                    break var;
                }
                var += 1;
            }
        }

        fn get_src_var(
            varmap: &mut HashMap<SrcVar, GenVar>,
            vars: &mut HashSet<GenVar>,
            var: SrcVar,
        ) -> usize {
            varmap.entry(var).or_insert(GenVar(new_gen_var(vars))).0
        }

        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        enum Dfs {
            Enter(Ast),
            EnterLoopable {
                repetitions_var: usize,
                ast: AstLoopable,
            },
            ExitBuild {
                repetitions_var: usize,
            },
            ExitForLoop {
                induction_var: usize,
                end_var: usize,
                repetitions_var: usize,
                length_var: usize,
                pop_count: usize,
                top_loop_jump_index: usize,
            },
        }
        let mut stack = vec![Dfs::Enter(self)];
        let mut current_index_elements: Vec<ConstantExpr> = Vec::with_capacity(32);
        let mut stmts: Vec<Stmt0> = Vec::with_capacity(64);

        while let Some(dfs) = stack.pop() {
            match dfs.clone() {
                Dfs::Enter(ast) => match ast {
                    Ast::Sym(key) => {
                        stmts.push(Stmt0::sym(key));
                    }
                    Ast::Copy(index) => {
                        let mut new_elements = current_index_elements.clone();
                        for element in index.elements() {
                            match element {
                                IndexElement::ZeroPlus(zp) => {
                                    new_elements.push(ConstantExpr::constant(*zp))
                                }
                                IndexElement::Var(var) => new_elements.push(ConstantExpr::var(
                                    get_src_var(&mut varmap, &mut vars, SrcVar(*var)),
                                )),
                                IndexElement::LenMinus(lm) => {
                                    let var = new_gen_var(&mut vars);
                                    stmts.extend([
                                        Stmt0::assign(var, Expr0::len(new_elements.clone())),
                                        Stmt0::assign(
                                            var,
                                            Expr0::sub(var, ConstantExpr::constant(*lm)),
                                        ),
                                    ]);
                                    new_elements.push(ConstantExpr::var(var));
                                }
                            }
                        }
                        stmts.push(Stmt0::copy(new_elements));
                    }
                    Ast::Build(b) => {
                        let repetitions_var = new_gen_var(&mut vars);
                        stmts.push(Stmt0::assign(repetitions_var, Expr0::constant(0)));
                        stack.push(Dfs::ExitBuild { repetitions_var });
                        stack.extend(b.into_iter().map(|ast| Dfs::EnterLoopable {
                            repetitions_var,
                            ast,
                        }));
                    }
                },
                Dfs::EnterLoopable {
                    repetitions_var,
                    ast,
                } => match ast {
                    AstLoopable::Ast(ast) => stack.push(Dfs::Enter(ast)),
                    AstLoopable::ForLoop {
                        var,
                        start,
                        end,
                        prior,
                        body,
                    } => {
                        let var = get_src_var(&mut varmap, &mut vars, SrcVar(var));
                        current_index_elements.extend(prior.iter().cloned());
                        let end_var = new_gen_var(&mut vars);
                        stmts.extend([
                            Stmt0::assign(var, Expr0::constant(start)),
                            Stmt0::assign(end_var, Expr0::len(current_index_elements.clone())),
                        ]);
                        current_index_elements.push(ConstantExpr::var(var));
                        if end.len_minus != 0 {
                            stmts.push(Stmt0::assign(
                                end_var,
                                Expr0::sub(end_var, ConstantExpr::constant(end.len_minus)),
                            ))
                        }
                        let length_var = new_gen_var(&mut vars);
                        stmts.push(Stmt0::assign(
                            length_var,
                            Expr0::sub(end_var, ConstantExpr::var(var)),
                        ));
                        let top_loop_jump_index = stmts.len();
                        stmts.push(Stmt0::unconditional_jump(0)); /* TODO */
                        stack.extend([
                            Dfs::ExitForLoop {
                                induction_var: var,
                                end_var,
                                repetitions_var,
                                length_var,
                                pop_count: prior.len().checked_add(1).unwrap(),
                                top_loop_jump_index,
                            },
                            Dfs::EnterLoopable {
                                repetitions_var,
                                ast: *body,
                            },
                        ]);
                    }
                },
                Dfs::ExitBuild { repetitions_var } => {
                    stmts.push(Stmt0::build(ConstantExpr::var(repetitions_var)))
                }
                Dfs::ExitForLoop {
                    induction_var,
                    end_var,
                    repetitions_var,
                    length_var,
                    mut pop_count,
                    top_loop_jump_index,
                } => {
                    stmts.push(Stmt0::assign(
                        induction_var,
                        Expr0::add(induction_var, ConstantExpr::constant(1)),
                    ));
                    let bot_loop_jump_index = stmts.len();
                    stmts.extend([
                        Stmt0::jump(
                            top_loop_jump_index.checked_add(1).unwrap(),
                            induction_var,
                            ConstantExpr::var(end_var),
                        ),
                        Stmt0::assign(
                            repetitions_var,
                            Expr0::add(repetitions_var, ConstantExpr::var(length_var)),
                        ),
                    ]);
                    match &mut stmts[top_loop_jump_index] {
                        Stmt0::UnconditionalJump(label) => {
                            label.0 = bot_loop_jump_index;
                        }
                        _ => unreachable!(),
                    };
                    while pop_count > 0 {
                        current_index_elements.pop();
                        pop_count = pop_count.checked_sub(1).unwrap();
                    }
                }
            }
            dbg!(dfs);
            println!("{}", Ast0::new(stmts.clone()));
            println!("-----------------");
        }

        Ast0::new(stmts)
    }
}

#[cfg(test)]
mod test {
    use crate::{lang0::expr::Var, storage::Storage};

    use super::*;

    #[test]
    fn compile0() {
        // (for x ((x ..) ..) -> (x .. ..))
        let ast = Ast::build(vec![AstLoopable::for_loop(
            lang0::expr::Var(0),
            0,
            0,
            vec![],
            AstLoopable::for_loop(
                lang0::expr::Var(1),
                0,
                0,
                vec![],
                AstLoopable::copy(vec![IndexElement::Var(0), IndexElement::Var(1)]),
            ),
        )]);
    }

    #[test]
    fn compile1() {
        // (for x ((x ..) ..) -> ((x ..) ..))
        let ast = Ast::build(vec![AstLoopable::for_loop(
            Var(0),
            0,
            0,
            vec![],
            AstLoopable::build(vec![AstLoopable::for_loop(
                Var(1),
                0,
                0,
                vec![],
                AstLoopable::copy(vec![IndexElement::Var(0), IndexElement::Var(1)]),
            )]),
        )]);
        let ast = ast.compile();
        let mut storage = Storage::new();
        let src = storage.read("((a b c) () (d) (e f g h i j k))").unwrap();
        storage.println(src, false);
        println!("{ast}");
        let result = ast.interpret(&mut storage, src);
        storage.println(result, false);
    }

    #[test]
    fn compile2() {
        // (for x (0 1 2 (x ..) 4 5 6) -> (x ..)
        let ast = Ast::build(vec![AstLoopable::for_loop(
            Var(0),
            0,
            0,
            vec![ConstantExpr::constant(3)],
            AstLoopable::copy(vec![IndexElement::Var(0)]),
        )]);
        let ast = ast.compile();
        let mut storage = Storage::new();
        let src = storage.read("(0 1 2 (a b c d e) 4 5 6)").unwrap();
        storage.println(src, false);
        println!("{ast}");
        let result = ast.interpret(&mut storage, src);
        storage.println(result, false);
    }
}
