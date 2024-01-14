use std::collections::VecDeque;

use crate::{
    compound::Compound,
    index::{
        zp_lookup, CompoundIndex, Index4, Index5, MiddleIndices, Nomiddle, NomiddleIndex,
        TermIndex, TermIndex1, TermIndexN,
    },
    storage::{Storage, StorageKey, Term},
    symbol::Symbol,
};

#[derive(Clone, Debug)]
pub enum SingleConstructor {
    Copy(NomiddleIndex),
    Symbol(String),
    Compound(Vec<CompoundElement>),
}

#[derive(Clone, Debug)]
struct CompoundElement {
    single_constructor: SingleConstructor,
    dot_dotted_indices: TermIndexN,
}

impl SingleConstructor {
    fn construct(&self, storage: &mut Storage, k: StorageKey) {
        #[derive(Debug)]
        enum Instruction {
            BuildCompoundTermOfLength(usize),
            ProcessConstructorWithOffsets {
                constructor: SingleConstructor,
                offsets: Vec<usize>,
            },
        }

        let mut instruction_stack = vec![Instruction::ProcessConstructorWithOffsets {
            constructor: self.clone(),
            offsets: vec![],
        }];

        let mut key_stack: Vec<StorageKey> = vec![];

        while let Some(instruction) = instruction_stack.pop() {
            // dbg!(&instruction);
            // for key in &key_stack {
            //     storage.println(*key, false);
            // }
            match instruction {
                Instruction::BuildCompoundTermOfLength(len) => {
                    let lower_bound = key_stack.len().checked_sub(len).unwrap();
                    let compount_term_elements: Vec<_> = key_stack.drain(lower_bound..).collect();
                    assert_eq!(len, compount_term_elements.len());
                    let key = storage.insert(Term::Compound(Compound::new(compount_term_elements)));
                    key_stack.push(key);
                }
                Instruction::ProcessConstructorWithOffsets {
                    constructor,
                    offsets,
                } => match constructor {
                    SingleConstructor::Symbol(s) => {
                        let k = storage.insert(Term::Symbol(Symbol::new(s)));
                        key_stack.push(k);
                    }
                    SingleConstructor::Copy(c) => {
                        let new_k = zp_lookup(&offsets, storage, k);
                        let new_k = c.lookup(storage, new_k);
                        key_stack.push(new_k);
                    }
                    SingleConstructor::Compound(c) => {
                        let mut pending_instructions: Vec<Instruction> = vec![];
                        for compound_element in c {
                            let constructor = compound_element.single_constructor;
                            let dot_dotted_indices = compound_element.dot_dotted_indices;
                            let dot_dotted_indices = TermIndexN::new(
                                offsets
                                    .iter()
                                    .map(|offset| CompoundIndex::ZeroPlus(*offset))
                                    .chain(dot_dotted_indices.compound_indices().iter().cloned())
                                    .collect(),
                            );
                            let dot_dotted_indices = dot_dotted_indices.to_index5();
                            match dot_dotted_indices {
                                Index5::WithMiddle(index4s) => {
                                    struct Elem {
                                        offsets: Vec<Nomiddle>,
                                    }
                                    let mut elem_stack: Vec<Elem> = vec![Elem { offsets: vec![] }];
                                    for index4 in index4s {
                                        elem_stack = elem_stack
                                            .drain(..)
                                            .flat_map(|elem| {
                                                let nomiddles: Vec<Nomiddle> = elem
                                                    .offsets
                                                    .iter()
                                                    .chain(index4.first.iter())
                                                    .cloned()
                                                    .collect();
                                                let zp = nomiddles_to_zp(&nomiddles, storage, k);
                                                let k = zp_lookup(&zp, storage, k);
                                                let repetitions =
                                                    index4.last.count_repetitions(storage, k);
                                                let starting_at_zp =
                                                    index4.last.starting_at_zero_plus();
                                                (0..repetitions).map(move |n| Elem {
                                                    offsets: nomiddles
                                                        .iter()
                                                        .chain(&[Nomiddle::ZeroPlus(
                                                            starting_at_zp + n,
                                                        )])
                                                        .cloned()
                                                        .collect(),
                                                })
                                            })
                                            .collect();
                                    }
                                    for elem in elem_stack {
                                        pending_instructions.push(
                                            Instruction::ProcessConstructorWithOffsets {
                                                constructor: constructor.clone(),
                                                offsets: nomiddles_to_zp(&elem.offsets, storage, k),
                                            },
                                        );
                                    }
                                }
                                Index5::WithoutMiddle(nomiddles) => {
                                    let offsets = nomiddles_to_zp(&nomiddles, storage, k);
                                    pending_instructions.push(
                                        Instruction::ProcessConstructorWithOffsets {
                                            constructor,
                                            offsets,
                                        },
                                    );
                                }
                            }
                        }
                        instruction_stack.push(Instruction::BuildCompoundTermOfLength(
                            pending_instructions.len(),
                        ));
                        instruction_stack.extend(pending_instructions.drain(..).rev());
                    }
                },
            }
        }

        assert_eq!(key_stack.len(), 1);
        let result = key_stack.pop().unwrap();
        let result = storage.get(result).unwrap().clone();
        storage.replace(k, result);
    }
}

fn nomiddles_to_zp(nomiddles: &[Nomiddle], storage: &Storage, mut k: StorageKey) -> Vec<usize> {
    nomiddles
        .iter()
        .map(|nomiddle| {
            let zp = match nomiddle {
                Nomiddle::ZeroPlus(zp) => *zp,
                Nomiddle::LenMinus(lm) => {
                    let len = storage.get_compound(k).unwrap().keys().len();
                    len.checked_sub(*lm).unwrap()
                }
            };
            k = storage.get_compound(k).unwrap().keys()[zp];
            zp
        })
        .collect()
}

#[cfg(test)]
mod test {
    use std::vec;

    use crate::index::Nomiddle;

    use super::*;

    #[test]
    fn construct0() {
        // (for x (x ..) -> (x ..))
        let constructor = SingleConstructor::Compound(vec![CompoundElement {
            single_constructor: SingleConstructor::Copy(NomiddleIndex::new(vec![])),
            dot_dotted_indices: TermIndexN::new(vec![CompoundIndex::Middle(MiddleIndices::new(
                0, 1,
            ))]),
        }]);
        let mut storage = Storage::new();
        let term = storage.read("(1 2 3 4 5 6)").unwrap();
        constructor.construct(&mut storage, term);
        let expected = storage.read("(1 2 3 4 5 6)").unwrap();
        assert!(storage.terms_are_equal(term, expected));
    }

    #[test]
    fn construct1() {
        // (for x (x ..) -> ((a x) ..))
        let constructor = SingleConstructor::Compound(vec![CompoundElement {
            single_constructor: SingleConstructor::Compound(vec![
                CompoundElement {
                    single_constructor: SingleConstructor::Symbol("a".into()),
                    dot_dotted_indices: TermIndexN::empty(),
                },
                CompoundElement {
                    single_constructor: SingleConstructor::Copy(NomiddleIndex::new(vec![])),
                    dot_dotted_indices: TermIndexN::empty(),
                },
            ]),
            dot_dotted_indices: TermIndexN::new(vec![CompoundIndex::Middle(MiddleIndices::new(
                0, 1,
            ))]),
        }]);
        let mut storage = Storage::new();
        let term = storage.read("(1 2 3 4 5 6)").unwrap();
        constructor.construct(&mut storage, term);
        let expected = storage
            .read("((a 1) (a 2) (a 3) (a 4) (a 5) (a 6))")
            .unwrap();
        assert!(storage.terms_are_equal(term, expected));
    }

    #[test]
    fn construct2() {
        // (for x y z ((x z .. y) ..) -> (x .. y .. z .. ..))
        let constructor = SingleConstructor::Compound(vec![
            CompoundElement {
                single_constructor: SingleConstructor::Copy(NomiddleIndex::new(vec![
                    Nomiddle::ZeroPlus(0),
                ])),
                dot_dotted_indices: TermIndexN::new(vec![CompoundIndex::Middle(
                    MiddleIndices::new(0, 1),
                )]),
            },
            CompoundElement {
                single_constructor: SingleConstructor::Copy(NomiddleIndex::new(vec![
                    Nomiddle::LenMinus(1),
                ])),
                dot_dotted_indices: TermIndexN::new(vec![CompoundIndex::Middle(
                    MiddleIndices::new(0, 1),
                )]),
            },
            CompoundElement {
                single_constructor: SingleConstructor::Copy(NomiddleIndex::new(vec![])),
                dot_dotted_indices: TermIndexN::new(vec![
                    CompoundIndex::Middle(MiddleIndices::new(0, 1)),
                    CompoundIndex::Middle(MiddleIndices::new(1, 2)),
                ]),
            },
        ]);

        let mut storage = Storage::new();
        let term = storage
            .read("((x0 z0a z0b y0) (x1 z1a z1b y1) (x2 z2a z2b y2))")
            .unwrap();
        constructor.construct(&mut storage, term);
        let expected = storage
            .read("(x0 x1 x2 y0 y1 y2 z0a z0b z1a z1b z2a z2b)")
            .unwrap();
        assert!(storage.terms_are_equal(term, expected));
    }

    #[test]
    fn construct3() {
        // (for x y z ((x z .. y) ..) -> ((y: y x: x z: (z ..)) ..))
        let constructor = SingleConstructor::Compound(vec![CompoundElement {
            single_constructor: SingleConstructor::Compound(vec![
                CompoundElement {
                    single_constructor: SingleConstructor::Symbol("y:".into()),
                    dot_dotted_indices: TermIndexN::empty(),
                },
                CompoundElement {
                    single_constructor: SingleConstructor::Copy(NomiddleIndex::new(vec![
                        Nomiddle::LenMinus(1),
                    ])),
                    dot_dotted_indices: TermIndexN::empty(),
                },
                CompoundElement {
                    single_constructor: SingleConstructor::Symbol("x:".into()),
                    dot_dotted_indices: TermIndexN::empty(),
                },
                CompoundElement {
                    single_constructor: SingleConstructor::Copy(NomiddleIndex::new(vec![
                        Nomiddle::ZeroPlus(0),
                    ])),
                    dot_dotted_indices: TermIndexN::empty(),
                },
                CompoundElement {
                    single_constructor: SingleConstructor::Symbol("z:".into()),
                    dot_dotted_indices: TermIndexN::empty(),
                },
                CompoundElement {
                    single_constructor: SingleConstructor::Compound(vec![CompoundElement {
                        single_constructor: SingleConstructor::Copy(NomiddleIndex::new(vec![])),
                        dot_dotted_indices: TermIndexN::new(vec![CompoundIndex::Middle(
                            MiddleIndices::new(1, 2),
                        )]),
                    }]),
                    dot_dotted_indices: TermIndexN::empty(),
                },
            ]),
            dot_dotted_indices: TermIndexN::new(vec![CompoundIndex::Middle(MiddleIndices::new(
                0, 1,
            ))]),
        }]);

        let mut storage = Storage::new();
        let term = storage
            .read("((x0 z0a z0b y0) (x1 z1a z1b y1) (x2 z2a z2b y2))")
            .unwrap();
        constructor.construct(&mut storage, term);
        let expected = storage
            .read("((y: y0 x: x0 z: (z0a z0b)) (y: y1 x: x1 z: (z1a z1b)) (y: y2 x: x2 z: (z2a z2b)))")
            .unwrap();
        storage.println(term, false);
        assert!(storage.terms_are_equal(term, expected));
    }
}
