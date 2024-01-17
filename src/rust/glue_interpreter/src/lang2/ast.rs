use std::collections::{HashMap, HashSet};

use crate::{
    index::ZeroPlus,
    lang0::{
        self,
        expr::{ConstantExpr, Var},
        stmt::Label,
    },
    lang2::varmap::{LangNPlusOneVar, LangNVar, Scope, Varmap},
    storage::StorageKey,
};
use lang0::ast::Ast as Ast0;
use lang0::expr::Expr as Expr0;
use lang0::stmt::Stmt as Stmt0;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Index {
    elements: Vec<IndexElement>,
}

impl Index {
    pub fn new(elements: Vec<IndexElement>) -> Self {
        Self { elements }
    }

    pub fn elements(&self) -> &[IndexElement] {
        &self.elements
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum IndexElement {
    ZeroPlus(usize),
    LenMinus(usize),
}

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
        var: LangNPlusOneVar,
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
            var: LangNPlusOneVar::new(var.0),
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
        let mut varm = Varmap::default();

        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        enum Dfs {
            Enter {
                ast: Ast,
                scope: Scope,
            },
            EnterLoopable {
                repetitions_var: LangNVar,
                ast: AstLoopable,
                scope: Scope,
            },
            ExitBuild {
                repetitions_var: LangNVar,
                scope: Scope,
            },
            ExitForLoop {
                induction_var: LangNVar,
                end_var: LangNVar,
                repetitions_var: LangNVar,
                length_var: LangNVar,
                pop_count: usize,
                top_loop_jump_index: usize,
                is_most_inner_loop: bool,
                scope: Scope,
            },
        }
        let mut stack = vec![Dfs::Enter {
            ast: self,
            scope: varm.enter_scope(),
        }];
        let mut current_index_elements: Vec<ConstantExpr> = Vec::with_capacity(32);
        let mut stmts: Vec<Stmt0> = Vec::with_capacity(64);

        fn get_len_minus_var(
            stmts: &mut Vec<Stmt0>,
            elements: Vec<ConstantExpr>,
            length_minus: usize,
            varm: &mut Varmap,
            scope: &Scope,
        ) -> usize {
            let var = varm.generate_var(scope);
            let var = var.into();
            stmts.extend([
                Stmt0::assign(var, Expr0::len(elements.clone())),
                Stmt0::assign(var, Expr0::sub(var, ConstantExpr::constant(length_minus))),
            ]);
            var
        }

        fn add_prior_elements(
            index: &Index,
            elements: &mut Vec<ConstantExpr>,
            stmts: &mut Vec<Stmt0>,
            varm: &mut Varmap,
            scope: &Scope,
        ) {
            for element in index.elements() {
                match element {
                    IndexElement::ZeroPlus(zp) => elements.push(ConstantExpr::constant(*zp)),
                    IndexElement::LenMinus(lm) => {
                        let var = get_len_minus_var(stmts, elements.clone(), *lm, varm, scope);
                        elements.push(ConstantExpr::var(var));
                    }
                }
            }
        }

        while let Some(dfs) = stack.pop() {
            match dfs.clone() {
                Dfs::Enter { ast, scope } => match ast {
                    Ast::Sym(key) => {
                        stmts.push(Stmt0::sym(key));
                    }
                    Ast::Copy(index) => {
                        let mut new_elements = current_index_elements.clone();
                        add_prior_elements(
                            &index,
                            &mut new_elements,
                            &mut stmts,
                            &mut varm,
                            &scope,
                        );
                        stmts.push(Stmt0::copy(new_elements));
                    }
                    Ast::Build(b) => {
                        let scope = varm.enter_scope();
                        let repetitions_var = varm.generate_var(&scope);
                        stmts.push(Stmt0::assign(repetitions_var.into(), Expr0::constant(0)));
                        stack.push(Dfs::ExitBuild {
                            repetitions_var,
                            scope,
                        });
                        stack.extend(b.into_iter().rev().map(|ast| Dfs::EnterLoopable {
                            repetitions_var,
                            ast,
                            scope,
                        }));
                    }
                },
                Dfs::EnterLoopable {
                    repetitions_var,
                    ast,
                    scope,
                } => match ast {
                    AstLoopable::Ast(ast) => stack.push(Dfs::Enter { ast, scope }),
                    AstLoopable::ForLoop {
                        var,
                        start,
                        end,
                        prior,
                        body,
                    } => {
                        let scope = varm.enter_scope();
                        let var = varm.get_source_variable(var, &scope);
                        add_prior_elements(
                            &prior,
                            &mut current_index_elements,
                            &mut stmts,
                            &mut varm,
                            &scope,
                        );
                        let end_var = varm.generate_var(&scope);
                        stmts.extend([
                            Stmt0::assign(var.into(), Expr0::constant(start)),
                            Stmt0::assign(
                                end_var.into(),
                                Expr0::len(current_index_elements.clone()),
                            ),
                        ]);
                        current_index_elements.push(ConstantExpr::var(var.into()));
                        if end.len_minus != 0 {
                            stmts.push(Stmt0::assign(
                                end_var.into(),
                                Expr0::sub(end_var.into(), ConstantExpr::constant(end.len_minus)),
                            ))
                        }
                        let length_var = varm.generate_var(&scope);
                        stmts.push(Stmt0::assign(
                            length_var.into(),
                            Expr0::sub(end_var.into(), ConstantExpr::var(var.into())),
                        ));
                        let top_loop_jump_index = stmts.len();
                        stmts.push(Stmt0::unconditional_jump(0)); /* TODO */
                        let is_most_inner_loop = match body.as_ref() {
                            AstLoopable::Ast(_) => true,
                            AstLoopable::ForLoop { .. } => false,
                        };
                        stack.extend([
                            Dfs::ExitForLoop {
                                induction_var: var,
                                end_var,
                                repetitions_var,
                                length_var,
                                pop_count: prior.elements().len().checked_add(1).unwrap(),
                                top_loop_jump_index,
                                is_most_inner_loop,
                                scope,
                            },
                            Dfs::EnterLoopable {
                                repetitions_var,
                                ast: *body,
                                scope,
                            },
                        ]);
                    }
                },
                Dfs::ExitBuild {
                    repetitions_var,
                    scope,
                } => {
                    stmts.push(Stmt0::build(ConstantExpr::var(repetitions_var.into())));
                    varm.exit_scope(scope);
                }
                Dfs::ExitForLoop {
                    induction_var,
                    end_var,
                    repetitions_var,
                    length_var,
                    mut pop_count,
                    top_loop_jump_index,
                    is_most_inner_loop,
                    scope,
                } => {
                    stmts.push(Stmt0::assign(
                        induction_var.into(),
                        Expr0::add(induction_var.into(), ConstantExpr::constant(1)),
                    ));
                    let bot_loop_jump_index = stmts.len();
                    stmts.push(Stmt0::jump(
                        top_loop_jump_index.checked_add(1).unwrap(),
                        induction_var.into(),
                        ConstantExpr::var(end_var.into()),
                    ));
                    if is_most_inner_loop {
                        stmts.push(Stmt0::assign(
                            repetitions_var.into(),
                            Expr0::add(
                                repetitions_var.into(),
                                ConstantExpr::var(length_var.into()),
                            ),
                        ));
                    }
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
                    varm.exit_scope(scope);
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
mod lang2_test {
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
        let src = storage
            .read("((x0 (y0 (z0))) (x1 (y1 (z1))) (x2 (())) ((y2 ())) (((z2))) ((())))")
            .unwrap();

        let src = storage
            .read("((x0 (y0 (z0))) (x1 (y1 (z1))) (x2 (())) ((y2 ())) (((z2))) (((0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 2 0))))")
            .unwrap();
        // let src = storage.read("((0a 1a 2a (3a 4a 5a 6a 7a (8a 9a))) (0b ((1b 2b))) (((0c 1c 2c))))").unwrap();
        storage.println(src, false);
        println!("{ast}");
        let result = ast.interpret(&mut storage, src);
        storage.println(result, false);
        let expected = storage.read("(x0 x1 x2 y0 y1 y2 z0 z1 z2 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 2 0)").unwrap();
        assert!(storage.terms_are_equal(result, expected));
    }
}
