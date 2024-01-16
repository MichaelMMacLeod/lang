use crate::lang0::expr::Var;

pub enum ExtendedConstantExpr {
    Var(Var),
    Constant(usize),
    
}