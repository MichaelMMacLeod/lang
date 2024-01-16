use std::collections::HashMap;

use crate::{lang0, storage::StorageKey};

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Ast {
    Copy(Index2),
    Sym(StorageKey),
    Build(Vec<Part>),
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Part {
    ast: Ast,
    dot_dotted_index: Index,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Index {
    Index2(Index2),
    Index3(Index3),
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Index2 {
    elements: Vec<Index2Element>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Index2Element {
    ZeroPlus(usize),
    LenMinus(usize),
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Index3 {
    elements: Vec<Index3Element>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Index3Element {
    first_elements: Vec<Index2Element>,
    last_element: Between,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Between {
    zero_plus: usize,
    len_minus: usize,
}

impl Ast {
    pub fn lower(self) -> lang0::ast::Ast {
        #[derive(Debug, PartialEq, Eq, Hash)]
        struct Index4 {
            first: lang0::expr::Index,
            last: Between,
        }

        let mut bindings: HashMap<Index4, lang0::expr::Var>;

        enum StackElem {
            Ast(Ast),
            Build {
                repetitions_var: lang0::expr::Var,
            },
            BuildLoop {
                repetitions_var: lang0::expr::Var,
                body: Ast,
            },
            Copy,
            Sym(StorageKey),
        }

        let mut stack = vec![StackElem::Ast(self)];

        while let Some(stack_elem) = stack.pop() {
            match stack_elem {
                StackElem::Ast(ast) => match ast {
                    Ast::Copy(c) => {}
                    Ast::Sym(_) => todo!(),
                    Ast::Build(_) => todo!(),
                },
                StackElem::Build { repetitions_var } => todo!(),
                StackElem::BuildLoop {
                    repetitions_var,
                    body,
                } => todo!(),
                StackElem::Copy => todo!(),
                StackElem::Sym(_) => todo!(),
            }
        }

        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn lower0() {
        // (for x ((x ..) ..) -> (x .. ..))
        let ast1 = Ast::Build(vec![Part {
            ast: Ast::Copy(Index2 { elements: vec![] }),
            dot_dotted_index: Index::Index3(Index3 {
                elements: vec![
                    Index3Element {
                        first_elements: vec![],
                        last_element: Between {
                            zero_plus: 0,
                            len_minus: 0,
                        },
                    },
                    Index3Element {
                        first_elements: vec![],
                        last_element: Between {
                            zero_plus: 0,
                            len_minus: 0,
                        },
                    },
                ],
            }),
        }]);
        // ( .... )
        //              #0 = 0
        //              ???
        //              build #0

        // input: Vec<ConstOrVar> = ...
        //        repetitionVar
        // (x ..)
        //              #0 = 0
        //              #1 = len [ ... ]
        //              jmp LOOP_BOT
        // LOOP_TOP:    copy [ ... #0 ]
        //              #0 = #0 + 1
        // LOOP_BOT:    jmp LOOP_TOP if #0 < #1
        //              build #1

        //              #0 = 0
        //              #1 = len [ ... ]
        //              #2 = 0
        //              jmp LOOP_BOT
        // LOOP_TOP:    ???.instructions
        //              #0 = #0 + 1
        //              #2 = #2 + ???.repetitions
        // LOOP_BOT:    jmp LOOP_TOP if #0 < #1
        //              build #2

        // (x .. ..)
        //              #0 = 0
        //              #1 = len [ ...0 ]
        //              #2 = 0
        //              jmp LOOP_0_BOT
        // LOOP_0_TOP:  #3 = 0
        //              #4 = len [ ...0 #0 ...1 ]
        //              jmp LOOP_1_BOT
        // LOOP_1_TOP:  copy [ ...0 #0 ...1 #3 ]
        //              #3 = #3 + 1
        // LOOP_1_BOT:  jmp LOOP_1_TOP if #3 < #4
        //              #0 = #0 + 1
        //              #2 = #2 + #4
        // LOOP_0_END:  jmp LOOP_0_TOP if #0 < #1
        //              build #2

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
    }
}
