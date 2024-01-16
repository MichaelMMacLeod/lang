use crate::storage::StorageKey;

use super::expr::{Expr, Index, Var, VarOrConstant};

#[derive(Debug)]
pub enum Stmt {
    Assign {
        lhs: Var,
        rhs: Expr,
    },
    Sym(StorageKey),
    Copy(Index),
    Build {
        length: VarOrConstant,
    },
    Jump {
        jump_to: Label,
        when_var: Var,
        le_var: VarOrConstant,
    },
    UnconditionalJump(Label),
}

#[derive(Debug)]
pub struct Label(pub usize);
