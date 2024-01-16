use crate::storage::{Storage, StorageKey};

#[derive(Debug, PartialEq, Eq)]
pub struct Var(pub usize);

impl Var {
    pub fn eval(&self, variables: &[usize]) -> usize {
        variables[self.0]
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ConstantExpr {
    Var(Var),
    Constant(usize),
}

impl ConstantExpr {
    pub fn var(v: usize) -> Self {
        Self::Var(Var(v))
    }

    pub fn constant(c: usize) -> Self {
        Self::Constant(c)
    }

    pub fn eval(&self, variables: &[usize]) -> usize {
        match self {
            ConstantExpr::Var(v) => v.eval(variables),
            ConstantExpr::Constant(c) => *c,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum OpExpr {
    ConstantExpr(ConstantExpr),
    Op {
        kind: OpKind,
        lhs: Var,
        rhs: ConstantExpr,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub enum OpKind {
    Add,
    Sub,
}

impl OpExpr {
    pub fn var(v: usize) -> Self {
        Self::ConstantExpr(ConstantExpr::var(v))
    }

    pub fn constant(c: usize) -> Self {
        Self::ConstantExpr(ConstantExpr::constant(c))
    }

    pub fn add(var: usize, e: ConstantExpr) -> Self {
        Self::Op {
            kind: OpKind::Add,
            lhs: Var(var),
            rhs: e,
        }
    }

    pub fn sub(var: usize, e: ConstantExpr) -> Self {
        Self::Op {
            kind: OpKind::Sub,
            lhs: Var(var),
            rhs: e,
        }
    }

    pub fn eval(&self, variables: &[usize]) -> usize {
        match self {
            OpExpr::ConstantExpr(ve) => ve.eval(variables),
            OpExpr::Op { kind, lhs, rhs } => {
                let lhs = lhs.eval(variables);
                let rhs = rhs.eval(variables);
                match kind {
                    OpKind::Add => lhs.checked_add(rhs).unwrap(),
                    OpKind::Sub => lhs.checked_sub(rhs).unwrap(),
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Expr {
    OpExpr(OpExpr),
    Len(Index),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Index {
    elements: Vec<ConstantExpr>,
}

impl Index {
    pub fn new(elements: Vec<ConstantExpr>) -> Self {
        Self { elements }
    }
}

impl Index {
    pub fn eval(&self, variables: &[usize], storage: &Storage, mut src: StorageKey) -> StorageKey {
        for index in &self.elements {
            let index = index.eval(&variables);
            src = storage.get_compound(src).unwrap().keys()[index];
        }
        src
    }
}

impl Expr {
    pub fn var(v: usize) -> Self {
        Self::OpExpr(OpExpr::var(v))
    }

    pub fn constant(c: usize) -> Self {
        Self::OpExpr(OpExpr::constant(c))
    }

    pub fn add(var: usize, e: ConstantExpr) -> Self {
        Self::OpExpr(OpExpr::add(var, e))
    }

    pub fn sub(var: usize, e: ConstantExpr) -> Self {
        Self::OpExpr(OpExpr::sub(var, e))
    }

    pub fn len(elements: Vec<ConstantExpr>) -> Self {
        Self::Len(Index { elements })
    }

    pub fn eval(&self, variables: &[usize], storage: &Storage, src: StorageKey) -> usize {
        match self {
            Expr::OpExpr(oe) => oe.eval(variables),
            Expr::Len(index) => storage
                .get_compound(index.eval(variables, storage, src))
                .unwrap()
                .keys()
                .len(),
        }
    }
}
