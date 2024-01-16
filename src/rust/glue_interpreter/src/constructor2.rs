use crate::{
    compound::Compound,
    constructor::SingleConstructor,
    index::Index6s,
    storage::{Storage, StorageKey, Term},
};

pub enum Expr {
    Var(VarExpr),
    Constant(ConstantExpr),
    // Index(IndexExpr),
    Length(LengthExpr),
    Subtract(SubtractExpr),
    Multiply(MultiplyExpr),
}

impl Expr {
    pub fn var(v: usize) -> Self {
        Self::Var(VarExpr { var: v })
    }

    pub fn constant(v: usize) -> Self {
        Self::Constant(ConstantExpr { constant: v })
    }

    // pub fn index(elements: Vec<Expr>) -> Self {
    //     Self::Index(IndexExpr { indices: elements })
    // }

    pub fn length(of: IndexExpr) -> Self {
        Self::Length(LengthExpr { of: Box::new(of) })
    }

    pub fn subtract(lhs: Expr, rhs: Expr) -> Self {
        Self::Subtract(SubtractExpr {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        })
    }

    pub fn multiply(lhs: Expr, rhs: Expr) -> Self {
        Self::Multiply(MultiplyExpr {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        })
    }
}

pub struct VarExpr {
    var: usize,
}

impl VarExpr {
    pub fn new(var: usize) -> Self {
        Self { var }
    }
}

pub struct ConstantExpr {
    constant: usize,
}

impl ConstantExpr {
    pub fn new(constant: usize) -> Self {
        Self { constant }
    }
}

pub struct LengthExpr {
    of: Box<IndexExpr>,
}

impl LengthExpr {
    pub fn new(of: Box<IndexExpr>) -> Self {
        Self { of }
    }
}

pub struct IndexExpr {
    indices: Vec<Expr>,
}

impl IndexExpr {
    pub fn new(indices: Vec<Expr>) -> Self {
        Self { indices }
    }
}

pub struct SubtractExpr {
    lhs: Box<Expr>,
    rhs: Box<Expr>,
}

impl SubtractExpr {
    pub fn new(lhs: Box<Expr>, rhs: Box<Expr>) -> Self {
        Self { lhs, rhs }
    }
}

pub struct MultiplyExpr {
    lhs: Box<Expr>,
    rhs: Box<Expr>,
}

impl MultiplyExpr {
    pub fn new(lhs: Box<Expr>, rhs: Box<Expr>) -> Self {
        Self { lhs, rhs }
    }
}

pub enum Instruction {
    Symbol(SymbolInstruction),
    Compound(CompoundInstruction),
    ForLoop(ForLoopInstruction),
    Copy(CopyInstruction),
}

impl Instruction {
    pub fn symbol(key: StorageKey) -> Self {
        Self::Symbol(SymbolInstruction { symbol: key })
    }

    pub fn compound(length: Expr) -> Self {
        Self::Compound(CompoundInstruction { length })
    }

    pub fn for_loop(var: usize, start: Expr, end: Expr, body: Vec<Instruction>) -> Self {
        Self::ForLoop(ForLoopInstruction {
            var: VarExpr { var },
            start,
            end,
            body,
        })
    }

    pub fn copy(src: IndexExpr) -> Self {
        Self::Copy(CopyInstruction { src })
    }
}

pub struct SymbolInstruction {
    symbol: StorageKey,
}

pub struct CompoundInstruction {
    length: Expr,
}

pub struct ForLoopInstruction {
    var: VarExpr,
    start: Expr,
    end: Expr,
    body: Vec<Instruction>,
}

pub struct CopyInstruction {
    src: IndexExpr,
}

pub struct Constructor {
    instructions: Vec<Instruction>,
}

impl Constructor {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Self { instructions }
    }
}

fn interpret(constructor: &Constructor, storage: &mut Storage, src: StorageKey) {
    let mut variables: Vec<usize> = Vec::new();
    let mut data_stack: Vec<StorageKey> = vec![];
    let mut instruction_stack: Vec<&[Instruction]> = Vec::new();

    instruction_stack.push(&constructor.instructions);

    while let Some(instructions) = instruction_stack.pop() {
        for instruction in instructions {
            match instruction {
                Instruction::Symbol(s) => {
                    data_stack.push(s.symbol);
                }
                Instruction::Compound(c) => {
                    let length = interpret_expr(&c.length, &variables, storage, src);
                    let data: Vec<_> = data_stack.drain(..length).rev().collect();
                    let term = storage.insert(Term::Compound(Compound::new(data)));
                    data_stack.push(term);
                }
                Instruction::ForLoop(f) => {
                    let start = interpret_expr(&f.start, &variables, storage, src);
                    let end = interpret_expr(&f.end, &variables, storage, src);
                    variables[f.var.var] = start;
                }
                Instruction::Copy(_) => todo!(),
            }
        }
    }

    assert_eq!(data_stack.len(), 1);
    let result = storage.get(data_stack.pop().unwrap()).unwrap().clone();
    storage.replace(src, result);
}

fn interpret_expr(expr: &Expr, variables: &[usize], storage: &Storage, src: StorageKey) -> usize {
    match expr {
        Expr::Var(v) => variables[v.var],
        Expr::Constant(c) => c.constant,
        Expr::Length(l) => {
            let src = interpret_index(&l.of, variables, storage, src);
            storage.get_compound(src).unwrap().keys().len()
        }
        Expr::Subtract(s) => {
            let lhs = interpret_expr(&s.lhs, variables, storage, src);
            let rhs = interpret_expr(&s.rhs, variables, storage, src);
            lhs.checked_sub(rhs).unwrap()
        }
        Expr::Multiply(m) => {
            let lhs = interpret_expr(&m.lhs, variables, storage, src);
            let rhs = interpret_expr(&m.rhs, variables, storage, src);
            lhs.checked_mul(rhs).unwrap()
        }
    }
}

fn interpret_index(
    expr: &IndexExpr,
    variables: &[usize],
    storage: &Storage,
    mut src: StorageKey,
) -> StorageKey {
    let index = expr
        .indices
        .iter()
        .map(move |expr| interpret_expr(expr, variables, storage, src));
    for i in index {
        src = storage.get_compound(src).unwrap().keys()[i];
    }
    src
}

#[cfg(test)]
mod test {
    use crate::{storage::Term, symbol::Symbol};

    use super::*;

    #[test]
    fn instr0() {
        // (for x (x ..) -> ((a x) ..))
        let mut storage = Storage::new();
        let a = storage.insert(Term::Symbol(Symbol::new("a".into())));
        let constructor = Constructor::new(vec![
            Instruction::for_loop(
                0,
                Expr::constant(0),
                Expr::subtract(Expr::length(IndexExpr::new(vec![])), Expr::constant(1)),
                vec![
                    Instruction::symbol(a),
                    Instruction::copy(IndexExpr::new(vec![Expr::var(0)])),
                    Instruction::compound(Expr::constant(2)),
                ],
            ),
            Instruction::compound(Expr::length(IndexExpr::new(vec![]))),
        ]);
    }
}

// pub struct Constructor2 {
//     instructions: Vec<Instruction>,
// }

// impl Constructor2 {
//     pub fn new(instructions: Vec<Instruction>) -> Self {
//         Self { instructions }
//     }
// }

// pub enum Instruction {
//     Symbol(StringInstruction),
//     Copy(CopyInstruction),
//     Begin(BeginInstruction),
//     End(EndInstruction),
// }

// impl Instruction {
//     pub fn symbol(string: String) -> Self {
//         Self::Symbol(StringInstruction { string })
//     }

//     pub fn copy(indices: Index6s) -> Self {
//         Self::Copy(CopyInstruction { indices })
//     }

//     pub fn compound() -> Self {
//         Self::Compound(CompoundInstruction)
//     }
// }

// pub struct StringInstruction {
//     string: String,
// }

// impl StringInstruction {
//     pub fn new(string: String) -> Self {
//         Self { string }
//     }
// }

// pub struct CopyInstruction {
//     indices: Index6s,
// }

// impl CopyInstruction {
//     pub fn new(indices: Index6s) -> Self {
//         Self { indices }
//     }
// }

// pub struct CompoundInstruction;

// mod test {
//     use crate::index::Index6;

//     use super::*;

//     #[test]
//     fn instr0() {
//         // (for x (x ..) -> (x ..))
//         let instructions = Constructor2::new(vec![
//             Instruction::begin(Index6s::new(vec![Index6::middle(0, 1)])),
//             Instruction::copy(),
//             Instruction::end(),
//         ]);
//     }

//     #[test]
//     fn instr1() {
//         // (for x (x ..) -> ((a x) ..))
//         let instructions = Constructor2::new(vec![
//             Instruction::begin(Index6s::new(vec![Index6::middle(0, 1)])),
//             Instruction::symbol("a".into()),
//             Instruction::copy(),
//             Instruction::end(),
//         ]);
//     }

//     #[test]
//     fn instr2() {
//         // (for y x (((g y) x ..) ..) -> ((a x ..) .. (y ..)))
//         let instructions = Constructor2::new(vec![
//             // for v0 in 0 to t.len-1
//             //   symbol a
//             //   for v1 in 1 to t[v0].len-1
//             //     copy v0 v1
//             //   end
//             //   compound len-1-1
//             // end
//             // for v0 in 0 to len-1
//             //   copy v0 0 1
//             // end
//             // compound len-1
//             // compound
//             Instruction::begin(Index6s::new(vec![Index6::middle(0, 1)])),
//             Instruction::begin(Index6s::new(vec![Index6::middle(1, 1)])),
//             Instruction::symbol("a".into()),
//             Instruction::copy(),
//             Instruction::end(),
//             Instruction::end(),
//         ]);
//     }
// }
