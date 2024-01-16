use crate::{
    compound::Compound,
    lang0::expr::VarOrConstant,
    storage::{Storage, StorageKey, Term},
};

use super::stmt::Stmt;

#[derive(Debug)]
pub struct Ast {
    stmts: Vec<Stmt>,
}

impl Ast {
    pub fn interpret(&self, storage: &mut Storage, src: StorageKey) -> StorageKey {
        let mut instruction_pointer: usize = 0;
        let mut variables: Vec<usize> = Vec::with_capacity(1024);
        let mut key_stack: Vec<StorageKey> = Vec::with_capacity(1024);

        while let Some(instruction) = self.stmts.get(instruction_pointer) {
            match instruction {
                Stmt::Assign { lhs, rhs } => {
                    while variables.get(lhs.0).is_none() {
                        variables.push(0);
                    }
                    variables[lhs.0] = rhs.eval(&variables, storage, src);
                    instruction_pointer += 1;
                }
                Stmt::Sym(s) => {
                    key_stack.push(*s);
                    instruction_pointer += 1;
                }
                Stmt::Copy(c) => {
                    key_stack.push(c.get(&variables, storage, src));
                    instruction_pointer += 1;
                }
                Stmt::Build { length } => {
                    let length = length.eval(&variables);
                    let lower_bound = key_stack.len().checked_sub(length).unwrap();
                    let data: Vec<_> = key_stack.drain(lower_bound..).collect();
                    let key = storage.insert(Term::Compound(Compound::new(data)));
                    key_stack.push(key);
                    instruction_pointer += 1;
                }
                Stmt::Jump {
                    jump_to,
                    when_var,
                    le_var,
                } => {
                    if when_var.get(&variables) < le_var.eval(&variables) {
                        instruction_pointer = jump_to.0;
                    } else {
                        instruction_pointer += 1;
                    }
                }
                Stmt::UnconditionalJump(label) => {
                    instruction_pointer = label.0;
                }
            }
            // dbg!(instruction);
            // dbg!(&variables);
            // println!("------------------------------");
        }

        assert_eq!(key_stack.len(), 1);
        key_stack.pop().unwrap()
    }
}

#[cfg(test)]
mod test {
    use crate::lang0::{
        expr::{Constant, Expr, Index, Var},
        stmt::Label,
    };

    use super::*;

    #[test]
    fn interpret0() {
        // (for x (x ..) -> (x ..))
        let mut storage = Storage::new();
        let src = storage.read("(a b c d e)").unwrap();
        let ast = Ast {
            stmts: vec![
                // 0
                Stmt::Assign {
                    lhs: Var(0),
                    rhs: Expr::Constant(0),
                },
                // 1
                Stmt::Assign {
                    lhs: Var(1),
                    rhs: Expr::Len(Index { elements: vec![] }),
                },
                // 2
                Stmt::Copy(Index {
                    elements: vec![VarOrConstant::Var(0)],
                }),
                // 3
                Stmt::Assign {
                    lhs: Var(0),
                    rhs: Expr::Add {
                        lhs: VarOrConstant::Var(0),
                        rhs: VarOrConstant::Constant(1),
                    },
                },
                // 4
                Stmt::Jump {
                    jump_to: Label(2),
                    when_var: Var(0),
                    le_var: VarOrConstant::Var(1),
                },
                // 5
                Stmt::Assign {
                    lhs: Var(3),
                    rhs: Expr::Len(Index { elements: vec![] }),
                },
                // 6
                Stmt::Build {
                    length: VarOrConstant::Var(3),
                },
            ],
        };
        let result = ast.interpret(&mut storage, src);
        let expected = storage.read("(a b c d e)").unwrap();
        storage.println(result, false);
        assert!(storage.terms_are_equal(result, expected));
    }

    #[test]
    fn interpret1() {
        // (for x (x ..) -> ((a x) ..))
        let mut storage = Storage::new();
        let src = storage.read("(a b c d e)").unwrap();
        let a = storage.read("a").unwrap();
        let ast = Ast {
            stmts: vec![
                Stmt::Assign {
                    lhs: Var(0),
                    rhs: Expr::Constant(0),
                },
                Stmt::Assign {
                    lhs: Var(1),
                    rhs: Expr::Len(Index { elements: vec![] }),
                },
                Stmt::Sym(a),
                Stmt::Copy(Index {
                    elements: vec![VarOrConstant::Var(0)],
                }),
                Stmt::Build {
                    length: VarOrConstant::Constant(2),
                },
                Stmt::Assign {
                    lhs: Var(0),
                    rhs: Expr::Add {
                        lhs: VarOrConstant::Var(0),
                        rhs: VarOrConstant::Constant(1),
                    },
                },
                Stmt::Jump {
                    jump_to: Label(2),
                    when_var: Var(0),
                    le_var: VarOrConstant::Var(1),
                },
                Stmt::Assign {
                    lhs: Var(3),
                    rhs: Expr::Len(Index { elements: vec![] }),
                },
                Stmt::Build {
                    length: VarOrConstant::Var(3),
                },
            ],
        };
        let result = ast.interpret(&mut storage, src);
        let expected = storage.read("((a a) (a b) (a c) (a d) (a e))").unwrap();
        storage.println(result, false);
        assert!(storage.terms_are_equal(result, expected));
    }

    #[test]
    fn interpret2() {
        // (for x ((x ..) ..) -> ((x ..) ..))
        let mut storage = Storage::new();
        let src = storage.read("((a b c d e) (f g) () (h i j) (k))").unwrap();
        let a = storage.read("a").unwrap();
        let ast = Ast {
            stmts: vec![
                //              #0 = 0
                //              #1 = len []
                //              jmp to LOOP_1_END
                // LOOP_1:      #2 = 0
                //              #3 = len [#0]
                //              jmp to LOOP_2_END
                // LOOP_2:      copy [#0 #2]
                //              #2 = #2 + 1
                // LOOP_2_END:  jmp to LOOP_2 if #2 < #3
                //              build #3
                //              #0 = #0 + 1
                // LOOP_1_END:  jmp to LOOP_1 if #0 < #1
                //              build #1
                Stmt::Assign {
                    lhs: Var(0),
                    rhs: Expr::Constant(0),
                },
                Stmt::Assign {
                    lhs: Var(1),
                    rhs: Expr::Len(Index { elements: vec![] }),
                },
                Stmt::UnconditionalJump(Label(11 /* LOOP_1_END */)),
                /* LOOP_1 = 3 */
                Stmt::Assign {
                    lhs: Var(2),
                    rhs: Expr::Constant(0),
                },
                Stmt::Assign {
                    lhs: Var(3),
                    rhs: Expr::Len(Index {
                        elements: vec![VarOrConstant::Var(0)],
                    }),
                },
                Stmt::UnconditionalJump(Label(8 /* LOOP_2_END */)),
                /* LOOP_2 = 6 */
                Stmt::Copy(Index {
                    elements: vec![VarOrConstant::Var(0), VarOrConstant::Var(2)],
                }),
                Stmt::Assign {
                    lhs: Var(2),
                    rhs: Expr::Add {
                        lhs: VarOrConstant::Var(2),
                        rhs: VarOrConstant::Constant(1),
                    },
                },
                /* LOOP_2_END = 8 */
                Stmt::Jump {
                    jump_to: Label(6 /* LOOP_2 */),
                    when_var: Var(2),
                    le_var: VarOrConstant::Var(3),
                },
                Stmt::Build {
                    length: VarOrConstant::Var(3),
                },
                Stmt::Assign {
                    lhs: Var(0),
                    rhs: Expr::Add {
                        lhs: VarOrConstant::Var(0),
                        rhs: VarOrConstant::Constant(1),
                    },
                },
                /* LOOP_1_END = 11 */
                Stmt::Jump {
                    jump_to: Label(3 /* LOOP_1 */),
                    when_var: Var(0),
                    le_var: VarOrConstant::Var(1),
                },
                Stmt::Build {
                    length: VarOrConstant::Var(1),
                },
            ],
        };
        let result = ast.interpret(&mut storage, src);
        let expected = storage.read("((a b c d e) (f g) () (h i j) (k))").unwrap();
        storage.println(result, false);
        assert!(storage.terms_are_equal(result, expected));
    }

    #[test]
    fn interpret3() {
        // (for x ((x ..) ..) -> (x .. ..))
        let mut storage = Storage::new();
        let src = storage.read("((a b c d e) (f g) () (h i j) (k))").unwrap();
        let ast = Ast {
            stmts: vec![
                //              #0 = 0
                //              #1 = len []
                //              #2 = 0
                //              jmp to LOOP_1_END
                // LOOP_1:      #3 = 0                      4
                //              #4 = len [#0]
                //              jmp to LOOP_2_END
                // LOOP_2:      copy [#0 #3]                7
                //              #3 = #3 + 1
                // LOOP_2_END:  jmp to LOOP_2 if #3 < #4    9
                //              #0 = #0 + 1
                //              #2 = #2 + #4
                // LOOP_1_END:  jmp to LOOP_1 if #0 < #1    12
                //              build #2
                Stmt::Assign {
                    lhs: Var(0),
                    rhs: Expr::Constant(0),
                },
                Stmt::Assign {
                    lhs: Var(1),
                    rhs: Expr::Len(Index { elements: vec![] }),
                },
                Stmt::Assign {
                    lhs: Var(2),
                    rhs: Expr::Constant(0),
                },
                Stmt::UnconditionalJump(Label(12)),
                Stmt::Assign {
                    lhs: Var(3),
                    rhs: Expr::Constant(0),
                },
                Stmt::Assign {
                    lhs: Var(4),
                    rhs: Expr::Len(Index {
                        elements: vec![VarOrConstant::Var(0)],
                    }),
                },
                Stmt::UnconditionalJump(Label(9)),
                Stmt::Copy(Index {
                    elements: vec![VarOrConstant::Var(0), VarOrConstant::Var(3)],
                }),
                Stmt::Assign {
                    lhs: Var(3),
                    rhs: Expr::Add {
                        lhs: VarOrConstant::Var(3),
                        rhs: VarOrConstant::Constant(1),
                    },
                },
                Stmt::Jump {
                    jump_to: Label(7),
                    when_var: Var(3),
                    le_var: VarOrConstant::Var(4),
                },
                Stmt::Assign {
                    lhs: Var(0),
                    rhs: Expr::Add {
                        lhs: VarOrConstant::Var(0),
                        rhs: VarOrConstant::Constant(1),
                    },
                },
                Stmt::Assign {
                    lhs: Var(2),
                    rhs: Expr::Add {
                        lhs: VarOrConstant::Var(2),
                        rhs: VarOrConstant::Var(4),
                    },
                },
                Stmt::Jump {
                    jump_to: Label(4),
                    when_var: Var(0),
                    le_var: VarOrConstant::Var(1),
                },
                Stmt::Build {
                    length: VarOrConstant::Var(2),
                },
            ],
        };
        let result = ast.interpret(&mut storage, src);
        let expected = storage.read("(a b c d e f g h i j k)").unwrap();
        storage.println(result, false);
        assert!(storage.terms_are_equal(result, expected));
    }
}
