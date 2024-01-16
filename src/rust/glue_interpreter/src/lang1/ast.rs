use std::collections::{HashMap, HashSet};

use crate::lang0;
use crate::lang0::expr::ConstantExpr;

use super::stmt::{IndexElement, Stmt};
use lang0::expr::Expr as Expr0;
use lang0::stmt::Stmt as Stmt0;

struct Ast {
    stmts: Vec<Stmt>,
}

impl Ast {
    fn compile(self) -> lang0::ast::Ast {
        let mut vars: HashSet<usize> = HashSet::new();
        let mut get_next_var = || {
            let mut var = vars.len();
            loop {
                if !vars.contains(&var) {
                    vars.insert(var);
                    break var;
                }
                var += 1;
            }
        };
        let stmts: Vec<lang0::stmt::Stmt> = self
            .stmts
            .into_iter()
            .flat_map(|stmt| {
                let mut lang0_stmts = Vec::new();
                match stmt {
                    Stmt::ForLoopPrologue {
                        induction_var,
                        start,
                        end_at_len_minus,
                        elements,
                    } => {
                        let end_var = get_next_var();
                        lang0_stmts.extend([
                            Stmt0::assign(induction_var.0, Expr0::constant(start)),
                            Stmt0::assign(end_var, Expr0::len(elements)),
                        ]);
                        if end_at_len_minus > 0 {
                            lang0_stmts.push(Stmt0::assign(
                                end_var,
                                Expr0::sub(end_var, ConstantExpr::constant(end_at_len_minus)),
                            ));
                        }
                        // lang0_stmts.push(Stmt0::unconditional_jump(destination_instruction));
                    }
                    Stmt::ForLoopEpilogue {
                        jump_to,
                        when_var,
                        le_var,
                    } => todo!(),
                    Stmt::BuildPrologue { repetition_var } => todo!(),
                    Stmt::BuildEpilogue { repetition } => todo!(),
                    Stmt::Sym(key) => lang0_stmts.push(Stmt0::sym(key)),
                    Stmt::Copy(index) => {
                        let mut constant_exprs: Vec<ConstantExpr> = Vec::new();
                        for element in index.elements() {
                            match element {
                                IndexElement::ZeroPlus(zp) => {
                                    constant_exprs.push(ConstantExpr::constant(*zp))
                                }
                                IndexElement::LenMinus(lm) => {
                                    let var = get_next_var();
                                    lang0_stmts.extend([
                                        Stmt0::assign(var, Expr0::len(constant_exprs.clone())),
                                        Stmt0::assign(
                                            var,
                                            Expr0::sub(var, ConstantExpr::constant(*lm)),
                                        ),
                                    ]);
                                    constant_exprs.push(ConstantExpr::var(var));
                                }
                                IndexElement::Var(v) => constant_exprs.push(ConstantExpr::var(*v)),
                            }
                        }
                        lang0_stmts.push(Stmt0::copy(constant_exprs));
                    }
                }
                lang0_stmts
            })
            .collect();

        lang0::ast::Ast::new(stmts)
    }
}
