use std::fmt::Display;

use crate::storage::StorageKey;

use super::expr::{ConstantExpr, Expr, Index, OpExpr, Var};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Stmt {
    Assign {
        lhs: Var,
        rhs: Expr,
    },
    Sym(StorageKey),
    Copy(Index),
    Build {
        length: ConstantExpr,
    },
    Jump {
        jump_to: Label,
        when_var: Var,
        le_var: ConstantExpr,
    },
    UnconditionalJump(Label),
}

impl Stmt {
    pub fn assign(var: usize, expr: Expr) -> Self {
        Self::Assign {
            lhs: Var(var),
            rhs: expr,
        }
    }

    pub fn sym(key: StorageKey) -> Self {
        Self::Sym(key)
    }

    pub fn copy(elements: Vec<ConstantExpr>) -> Self {
        Self::Copy(Index::new(elements))
    }

    pub fn build(length: ConstantExpr) -> Self {
        Self::Build { length }
    }

    pub fn jump(
        destination_instruction: usize,
        when_var: usize,
        is_less_than: ConstantExpr,
    ) -> Self {
        Self::Jump {
            jump_to: Label(destination_instruction),
            when_var: Var(when_var),
            le_var: is_less_than,
        }
    }

    pub fn unconditional_jump(destination_instruction: usize) -> Self {
        Self::UnconditionalJump(Label(destination_instruction))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Label(pub usize);

impl Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "@{}", self.0)
    }
}

impl Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Stmt::Assign { lhs, rhs } => {
                write!(f, "{} = {}", lhs, rhs)
            }
            Stmt::Sym(s) => {
                write!(f, "sym {:?}", s)
            }
            Stmt::Copy(c) => {
                write!(f, "copy {}", c)
            }
            Stmt::Build { length } => {
                write!(f, "build {}", length)
            }
            Stmt::Jump {
                jump_to,
                when_var,
                le_var,
            } => {
                write!(f, "jump {} when {} < {}", jump_to, when_var, le_var)
            }
            Stmt::UnconditionalJump(label) => {
                write!(f, "jump {}", label)
            }
        }
    }
}
