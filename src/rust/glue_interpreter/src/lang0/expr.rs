use crate::storage::{Storage, StorageKey};

#[derive(Debug)]
pub enum VarOrConstant {
    Constant(usize),
    Var(usize),
}

impl VarOrConstant {
    pub fn eval(&self, variables: &[usize]) -> usize {
        match self {
            VarOrConstant::Var(v) => variables[*v],
            VarOrConstant::Constant(c) => *c,
        }
    }
}

#[derive(Debug)]
pub enum Expr {
    Var(usize),
    Constant(usize),
    Len(Index),
    Add {
        lhs: VarOrConstant,
        rhs: VarOrConstant,
    },
    Sub {
        lhs: VarOrConstant,
        rhs: VarOrConstant,
    },
    Mul {
        lhs: VarOrConstant,
        rhs: VarOrConstant,
    },
}

impl Expr {
    pub fn eval(&self, variables: &[usize], storage: &Storage, src: StorageKey) -> usize {
        match self {
            Expr::Var(v) => variables[*v],
            Expr::Constant(c) => *c,
            Expr::Len(l) => {
                let key = l.get(variables, storage, src);
                storage.get_compound(key).unwrap().keys().len()
            }
            Expr::Add { lhs, rhs } => {
                let lhs = lhs.eval(variables);
                let rhs = rhs.eval(variables);
                lhs.checked_add(rhs).unwrap()
            }
            Expr::Sub { lhs, rhs } => {
                let lhs = lhs.eval(variables);
                let rhs = rhs.eval(variables);
                lhs.checked_sub(rhs).unwrap()
            }
            Expr::Mul { lhs, rhs } => {
                let lhs = lhs.eval(variables);
                let rhs = rhs.eval(variables);
                lhs.checked_mul(rhs).unwrap()
            }
        }
    }
}

#[derive(Debug)]
pub struct Len(pub Index);

#[derive(Debug)]
pub struct Add {
    lhs: Box<VarOrConstant>,
    rhs: Box<VarOrConstant>,
}

#[derive(Debug)]
pub struct Sub {
    lhs: Box<VarOrConstant>,
    rhs: Box<VarOrConstant>,
}

#[derive(Debug)]
pub struct Mult {
    lhs: Box<VarOrConstant>,
    rhs: Box<VarOrConstant>,
}

#[derive(Debug)]
pub struct Var(pub usize);

impl Var {
    pub fn get(&self, variables: &[usize]) -> usize {
        variables[self.0]
    }
}

#[derive(Debug)]
pub struct Constant(pub usize);

#[derive(Debug)]
pub struct Index {
    pub elements: Vec<VarOrConstant>,
}

impl Index {
    pub fn get(&self, variables: &[usize], storage: &Storage, mut src: StorageKey) -> StorageKey {
        for elem in &self.elements {
            let index = elem.eval(&variables);
            src = storage.get_compound(src).unwrap().keys()[index];
        }
        src
    }
}
