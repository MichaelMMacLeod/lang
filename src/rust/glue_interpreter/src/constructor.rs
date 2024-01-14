use std::collections::VecDeque;

use crate::{
    compound::Compound,
    index::{CompoundIndex, MiddleIndices, TermIndex, TermIndex1, TermIndexN},
    storage::{Storage, StorageKey, Term},
    symbol::Symbol,
};

#[derive(Clone)]
pub enum SingleConstructor {
    Copy(TermIndexN),
    Symbol(String),
    Compound(Vec<CompoundElement>),
}

#[derive(Clone)]
struct CompoundElement {
    single_constructor: SingleConstructor,
    dot_dot_count: usize,
}

impl SingleConstructor {
    fn shared_term_index_n(&self, dot_dot_count: usize) -> TermIndexN {
        let mut stack = vec![self.clone()];
        let indices: Vec<CompoundIndex> = loop {
            match stack.pop().unwrap() {
                SingleConstructor::Copy(c) => {
                    let mut result: Vec<CompoundIndex> = vec![];
                    let mut middle_count: usize = 0;
                    for compound_index in c.compound_indices() {
                        if middle_count == dot_dot_count {
                            break;
                        }
                        result.push(compound_index.clone());
                        if let CompoundIndex::Middle(_) = compound_index {
                            middle_count += 1;
                        }
                    }
                    assert!(middle_count == dot_dot_count);
                    break result;
                }
                SingleConstructor::Symbol(_) => {}
                SingleConstructor::Compound(c) => {
                    for compound_element in &c {
                        stack.push(compound_element.single_constructor.clone())
                    }
                }
            }
        };
        TermIndexN::new(indices)
    }

    fn construct(&self, storage: &mut Storage, k: StorageKey) {
        enum Instruction {
            Create(Instruction2),
            Populate(usize),
        }

        struct Instruction2 {
            offsets: Vec<usize>,
            constructor: SingleConstructor,
        }

        let destination = storage.insert(Term::Symbol(Symbol::new("".into() /* temp */)));

        let mut instruction_stack: Vec<Instruction> = vec![Instruction::Create(Instruction2 {
            offsets: vec![],
            constructor: self.clone(),
        })];

        let mut data_stack: Vec<StorageKey> = vec![];

        while let Some(instruction) = instruction_stack.pop() {
            match instruction {
                Instruction::Populate(size) => {
                    let keys: Vec<StorageKey> = data_stack.drain(0..size).collect();
                    assert_eq!(keys.len(), size);
                    let compound_key = storage.insert(Term::Compound(Compound::new(keys)));
                    data_stack.push(compound_key);
                },
                Instruction::Create(Instruction2 {
                    offsets,
                    constructor,
                }) => match constructor {
                    SingleConstructor::Copy(i) => {
                        let index_to_copy = i.into_term_index(storage, k, &offsets);
                        let key_to_copy = index_to_copy.lookup(storage, k);
                        data_stack.push(key_to_copy);
                    }
                    SingleConstructor::Symbol(s) => {
                        let key = storage.insert(Term::Symbol(Symbol::new(s)));
                        data_stack.push(key);
                    }
                    SingleConstructor::Compound(c) => {
                        fn next_repetition_count(
                            shared_indices: &TermIndexN,
                            storage: &Storage,
                            mut key: StorageKey,
                            offsets: &[usize],
                        ) -> usize {
                            let term_index_1: TermIndex1 = {
                                // let mut iter = shared_indices.compound_indices().iter();
                                let mut offsets_iter = offsets.into_iter();
                                let mut last: Option<MiddleIndices> = None;
                                let initial: Vec<usize> = shared_indices
                                    .compound_indices()
                                    .iter()
                                    .map_while(|index| match index {
                                        CompoundIndex::ZeroPlus(zp) => {
                                            key = storage.get_compound(key).unwrap().keys()[*zp];
                                            Some(*zp)
                                        }
                                        CompoundIndex::LengthMinus(lm) => {
                                            let len =
                                                storage.get_compound(key).unwrap().keys().len();
                                            let zp = len.checked_sub(*lm).unwrap();
                                            key = storage.get_compound(key).unwrap().keys()[zp];
                                            Some(zp)
                                        }
                                        CompoundIndex::Middle(m) => {
                                            if let Some(offset) = offsets_iter.next() {
                                                let zp = m
                                                    .starting_at_zero_plus()
                                                    .checked_add(*offset)
                                                    .unwrap();
                                                key = storage.get_compound(key).unwrap().keys()[zp];
                                                Some(zp)
                                            } else {
                                                last = Some(m.clone());
                                                None
                                            }
                                        }
                                    })
                                    .collect();
                                TermIndex1::new(initial, last.unwrap())
                            };
                            term_index_1.count_repetitions(storage, key)
                        }

                        for compound_element in c {
                            let shared_indices = compound_element
                                .single_constructor
                                .shared_term_index_n(compound_element.dot_dot_count);

                            #[derive(Debug)]
                            struct Elem {
                                offsets: Vec<usize>,
                            }

                            let mut queue: VecDeque<Elem> = VecDeque::new();
                            for offset in 0..next_repetition_count(&shared_indices, storage, k, &[])
                            {
                                queue.push_back(Elem {
                                    offsets: offsets.iter().chain(&[offset]).cloned().collect(),
                                })
                            }

                            while let Some(elem) = queue.pop_front() {
                                dbg!(&elem, queue.len());
                                if elem.offsets.len() >= compound_element.dot_dot_count {
                                    assert!(elem.offsets.len() == compound_element.dot_dot_count);
                                    // instruction_stack.push(Instruction::Create(()))
                                    // instruction_stack.push(Instruction {
                                    //     offsets: elem.offsets,
                                    //     destination,
                                    // });
                                } else {
                                    for offset in 0..next_repetition_count(
                                        &shared_indices,
                                        storage,
                                        k,
                                        &elem.offsets,
                                    ) {
                                        queue.push_back(Elem {
                                            offsets: elem
                                                .offsets
                                                .iter()
                                                .chain(&[offset])
                                                .cloned()
                                                .collect(),
                                        })
                                    }
                                }
                            }
                        }
                    }
                },
            }
            // match self {
            //     SingleConstructor::Copy(i) => {
            //         let index_to_copy = i.into_term_index(storage, k, &instruction.offsets);
            //         let key_to_copy = index_to_copy.lookup(storage, k);
            //         let copied_term = storage.get(key_to_copy).unwrap().clone();
            //         storage.replace(instruction.destination, copied_term);
            //     }
            //     SingleConstructor::Symbol(s) => {
            //         let copied_term = Term::Symbol(Symbol::new(s.clone()));
            //         storage.replace(instruction.destination, copied_term);
            //     }
            //     SingleConstructor::Compound(c) =>
            //     }
            // }
        }

        let destination = storage.get(destination).unwrap().clone();
        storage.replace(k, destination);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn construct0() {
        // (for x (x ..) -> (x ..))
        let constructor = SingleConstructor::Compound(vec![CompoundElement {
            single_constructor: SingleConstructor::Copy(TermIndexN::new(vec![
                CompoundIndex::Middle(MiddleIndices::new(0, 1)),
            ])),
            dot_dot_count: 1,
        }]);
        let mut storage = Storage::new();
        let term = storage.read("(a b c 1 2 3 4 5 6 d e)").unwrap();
        constructor.construct(&mut storage, term);
        storage.println(term, false);
    }

    #[test]
    fn construct1() {
        // (for x (a b c x .. d e) -> (x ..))
        let constructor = SingleConstructor::Compound(vec![CompoundElement {
            single_constructor: SingleConstructor::Copy(TermIndexN::new(vec![
                CompoundIndex::Middle(MiddleIndices::new(3, 3)),
            ])),
            dot_dot_count: 1,
        }]);
        let mut storage = Storage::new();
        let term = storage.read("(a b c 1 2 3 4 5 6 d e)").unwrap();
        constructor.construct(&mut storage, term);
        storage.println(term, false);
    }

    #[test]
    fn construct2() {
        // (for x (a b c x .. d e) -> true)
        let constructor = SingleConstructor::Symbol("true".into());
        let mut storage = Storage::new();
        let term = storage.read("(a b c 1 2 3 4 5 6 d e)").unwrap();
        constructor.construct(&mut storage, term);
        storage.println(term, false);
    }

    #[test]
    fn construct3() {
        // (for x (a b c x d e) -> x)
        let constructor =
            SingleConstructor::Copy(TermIndexN::new(vec![CompoundIndex::ZeroPlus(3)]));
        let mut storage = Storage::new();
        let term = storage.read("(a b c 1 d e)").unwrap();
        constructor.construct(&mut storage, term);
        storage.println(term, false);
    }
}
