use crate::{
    lang0::{
        expr::{ConstantExpr, Var},
        stmt::Label,
    },
    storage::StorageKey,
};

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
    Var(usize),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Stmt {
    ForLoopPrologue {
        induction_var: Var,
        start: usize,
        end_at_len_minus: usize,
        elements: Vec<ConstantExpr>,
    },
    ForLoopEpilogue {
        jump_to: Label,
        when_var: Var,
        le_var: ConstantExpr,
    },
    BuildPrologue {
        repetition_var: Option<Var>,
    },
    BuildEpilogue {
        repetition: ConstantExpr,
    },
    Sym(StorageKey),
    Copy(Index),
}