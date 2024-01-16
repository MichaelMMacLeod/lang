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
        prior: Index,
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
        prior: Vec<IndexElement>,
        body: AstLoopable,
    ) -> Self {
        Self::ForLoop {
            var: var.0,
            start,
            end: LoopEnd {
                len_minus: end_at_len_minus,
            },
            body: Box::new(body),
            prior: Index::new(prior),
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

        fn get_len_minus_var(
            stmts: &mut Vec<Stmt0>,
            vars: &mut HashSet<GenVar>,
            elements: Vec<ConstantExpr>,
            length_minus: usize,
        ) -> usize {
            let var = new_gen_var(vars);
            stmts.extend([
                Stmt0::assign(var, Expr0::len(elements.clone())),
                Stmt0::assign(var, Expr0::sub(var, ConstantExpr::constant(length_minus))),
            ]);
            var
        }

        fn add_prior_elements(
            index: &Index,
            elements: &mut Vec<ConstantExpr>,
            varmap: &mut HashMap<SrcVar, GenVar>,
            mut vars: &mut HashSet<GenVar>,
            stmts: &mut Vec<Stmt0>,
        ) {
            for element in index.elements() {
                match element {
                    IndexElement::ZeroPlus(zp) => elements.push(ConstantExpr::constant(*zp)),
                    IndexElement::Var(var) => elements.push(ConstantExpr::var(get_src_var(
                        varmap,
                        &mut vars,
                        SrcVar(*var),
                    ))),
                    IndexElement::LenMinus(lm) => {
                        let var = get_len_minus_var(stmts, &mut vars, elements.clone(), *lm);
                        elements.push(ConstantExpr::var(var));
                    }
                }
            }
        }

        while let Some(dfs) = stack.pop() {
            match dfs.clone() {
                Dfs::Enter(ast) => match ast {
                    Ast::Sym(key) => {
                        stmts.push(Stmt0::sym(key));
                    }
                    Ast::Copy(index) => {
                        let mut new_elements = current_index_elements.clone();
                        add_prior_elements(
                            &index,
                            &mut new_elements,
                            &mut varmap,
                            &mut vars,
                            &mut stmts,
                        );
                        stmts.push(Stmt0::copy(new_elements));
                    }
                    Ast::Build(b) => {
                        let repetitions_var = new_gen_var(&mut vars);
                        stmts.push(Stmt0::assign(repetitions_var, Expr0::constant(0)));
                        stack.push(Dfs::ExitBuild { repetitions_var });
                        stack.extend(b.into_iter().rev().map(|ast| Dfs::EnterLoopable {
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
                        add_prior_elements(
                            &prior,
                            &mut current_index_elements,
                            &mut varmap,
                            &mut vars,
                            &mut stmts,
                        );
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
                                pop_count: prior.elements().len().checked_add(1).unwrap(),
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
            // dbg!(dfs);
            // println!("{}", Ast0::new(stmts.clone()));
            // println!("-----------------");
        }

        Ast0::new(stmts)
    }
}

#[cfg(test)]
mod test {
    use crate::{lang0::expr::Var, storage::Storage};

    use super::*;

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
                AstLoopable::copy(vec![]),
            )]),
        )]);
        let ast = ast.compile();
        let mut storage = Storage::new();
        let src = storage.read("((a b c) () (d) (e f g h i j k))").unwrap();
        storage.println(src, false);
        println!("{ast}");
        let result = ast.interpret(&mut storage, src);
        storage.println(result, false);
        let expected = storage.read("((a b c) () (d) (e f g h i j k))").unwrap();
        assert!(storage.terms_are_equal(result, expected));
    }

    #[test]
    fn compile2() {
        // (for x (0 1 2 (x ..) 4 5 6) -> (x ..)
        let ast = Ast::build(vec![AstLoopable::for_loop(
            Var(0),
            0,
            0,
            vec![IndexElement::ZeroPlus(3)],
            AstLoopable::copy(vec![]),
        )]);
        let ast = ast.compile();
        let mut storage = Storage::new();
        let src = storage.read("(0 1 2 (a b c d e) 4 5 6)").unwrap();
        storage.println(src, false);
        println!("{ast}");
        let result = ast.interpret(&mut storage, src);
        storage.println(result, false);
        let expected = storage.read("(a b c d e)").unwrap();
        assert!(storage.terms_are_equal(result, expected));
    }

    #[test]
    fn compile3() {
        // (for x (0 1 (20 (x ..) 22 23 24) 3) -> (x ..)
        let ast = Ast::build(vec![AstLoopable::for_loop(
            Var(0),
            0,
            0,
            vec![IndexElement::ZeroPlus(2), IndexElement::ZeroPlus(1)],
            AstLoopable::copy(vec![]),
        )]);
        let ast = ast.compile();
        let mut storage = Storage::new();
        let src = storage.read("(0 1 (20 (a b c d e) 22 23 24) 3)").unwrap();
        storage.println(src, false);
        println!("{ast}");
        let result = ast.interpret(&mut storage, src);
        storage.println(result, false);
        let expected = storage.read("(a b c d e)").unwrap();
        assert!(storage.terms_are_equal(result, expected));
    }

    #[test]
    fn compile4() {
        // (for y x (0 1 (20 (y .. x) 22 23 24) 3) -> x
        let ast = Ast::copy(vec![
            IndexElement::ZeroPlus(2),
            IndexElement::ZeroPlus(1),
            IndexElement::LenMinus(1),
        ]);
        let ast = ast.compile();
        let mut storage = Storage::new();
        let src = storage.read("(0 1 (20 (a b c d e) 22 23 24) 3)").unwrap();
        storage.println(src, false);
        println!("{ast}");
        let result = ast.interpret(&mut storage, src);
        storage.println(result, false);
        let expected = storage.read("e").unwrap();
        assert!(storage.terms_are_equal(result, expected));
    }

    #[test]
    fn compile5() {
        // (for x y z (x .. (y .. 0)) -> (x .. y ..))
        let ast = Ast::build(vec![
            AstLoopable::for_loop(Var(0), 0, 1, vec![], AstLoopable::copy(vec![])),
            AstLoopable::for_loop(
                Var(0),
                0,
                1,
                vec![IndexElement::LenMinus(1)],
                AstLoopable::copy(vec![]),
            ),
        ]);
        let ast = ast.compile();
        let mut storage = Storage::new();
        let src = storage.read("(0 1 2 (3 4 5 6 7 0))").unwrap();
        storage.println(src, false);
        println!("{ast}");
        let result = ast.interpret(&mut storage, src);
        storage.println(result, false);
        let expected = storage.read("(0 1 2 3 4 5 6 7)").unwrap();
        assert!(storage.terms_are_equal(result, expected));
    }

    #[test]
    fn compile6() {
        // (for x y z (x .. (y .. (z ..))) -> (x .. y .. z ..))
        let ast = Ast::build(vec![
            AstLoopable::for_loop(Var(0), 0, 1, vec![], AstLoopable::copy(vec![])),
            AstLoopable::for_loop(
                Var(0),
                0,
                1,
                vec![IndexElement::LenMinus(1)],
                AstLoopable::copy(vec![]),
            ),
            AstLoopable::for_loop(
                Var(0),
                0,
                0,
                vec![IndexElement::LenMinus(1), IndexElement::LenMinus(1)],
                AstLoopable::copy(vec![]),
            ),
        ]);
        let ast = ast.compile();
        let mut storage = Storage::new();
        let src = storage.read("(0 1 2 (3 4 5 6 7 (8 9)))").unwrap();
        storage.println(src, false);
        println!("{ast}");
        let result = ast.interpret(&mut storage, src);
        storage.println(result, false);
        let expected = storage.read("(0 1 2 3 4 5 6 7 8 9)").unwrap();
        assert!(storage.terms_are_equal(result, expected));
    }

    #[test]
    fn compile7() {
        // (for x y z ((x .. (y .. (z ..))) ..) -> (x .. .. y .. .. z .. ..))
        let ast = Ast::build(vec![
            AstLoopable::for_loop(
                Var(0),
                0,
                0,
                vec![],
                AstLoopable::for_loop(Var(1), 0, 1, vec![], AstLoopable::copy(vec![])),
            ),
            AstLoopable::for_loop(
                Var(0),
                0,
                0,
                vec![],
                AstLoopable::for_loop(
                    Var(1),
                    0,
                    1,
                    vec![IndexElement::LenMinus(1)],
                    AstLoopable::copy(vec![]),
                ),
            ),
            AstLoopable::for_loop(
                Var(0),
                0,
                0,
                vec![],
                AstLoopable::for_loop(
                    Var(1),
                    0,
                    0,
                    vec![IndexElement::LenMinus(1), IndexElement::LenMinus(1)],
                    AstLoopable::copy(vec![]),
                ),
            ),
        ]);
        let ast = ast.compile();
        let mut storage = Storage::new();
        let src = storage.read("((x0 (y0 (z0))) (x1 (y1 (z1))))").unwrap();
        // let src = storage.read("((0a 1a 2a (3a 4a 5a 6a 7a (8a 9a))) (0b ((1b 2b))) (((0c 1c 2c))))").unwrap();
        storage.println(src, false);
        println!("{ast}");
        let result = ast.interpret(&mut storage, src);
        storage.println(result, false);
        let expected = storage.read("(x0 y0 z0 x1 y1 z1)").unwrap();
        assert!(storage.terms_are_equal(result, expected));
    }
}
