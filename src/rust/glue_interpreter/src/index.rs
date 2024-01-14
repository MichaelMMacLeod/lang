use crate::storage::{Storage, StorageKey, Term};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum CompoundIndex {
    ZeroPlus(usize),
    LengthMinus(usize),
    Middle(MiddleIndices),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MiddleIndices {
    starting_at_zero_plus: usize,
    ending_at_length_minus: usize,
}

impl MiddleIndices {
    pub fn new(starting_at_zero_plus: usize, ending_at_length_minus: usize) -> Self {
        Self {
            starting_at_zero_plus,
            ending_at_length_minus,
        }
    }

    pub fn starting_at_zero_plus(&self) -> usize {
        self.starting_at_zero_plus
    }

    pub fn ending_at_length_minus(&self) -> usize {
        self.ending_at_length_minus
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TermIndexN {
    compound_indices: Vec<CompoundIndex>,
}

impl TermIndexN {
    pub fn new(indices: Vec<CompoundIndex>) -> Self {
        Self {
            compound_indices: indices,
        }
    }

    pub fn compound_indices(&self) -> &[CompoundIndex] {
        &self.compound_indices
    }

    pub fn into_term_index(
        &self,
        storage: &Storage,
        mut k: StorageKey,
        offsets: &[usize],
    ) -> TermIndex {
        let mut indices = Vec::new();

        let mut offsets = offsets.into_iter();
        for index_n in &self.compound_indices {
            let zp = match index_n {
                CompoundIndex::ZeroPlus(zp) => *zp,
                CompoundIndex::LengthMinus(lm) => {
                    let len = storage.get_compound(k).unwrap().keys().len();
                    len.checked_sub(*lm).unwrap()
                }
                CompoundIndex::Middle(m) => {
                    let &offset = offsets.next().unwrap();
                    m.starting_at_zero_plus().checked_add(offset).unwrap()
                }
            };
            k = storage.get_compound(k).unwrap().keys()[zp];
            indices.push(zp);
        }

        assert!(offsets.next().is_none());

        TermIndex::new(indices)
    }

    fn lookup(&self, storage: &Storage, k: StorageKey) -> Vec<StorageKey> {
        let mut result = vec![k];
        let mut middle_buffer = Vec::new();
        for index in &self.compound_indices {
            match index {
                CompoundIndex::ZeroPlus(zp) => {
                    for key in &mut result {
                        let keys = match storage.get(*key).unwrap() {
                            Term::Compound(c) => c.keys(),
                            _ => panic!("attempt to index into non-compound term"),
                        };
                        *key = keys[*zp];
                    }
                }
                CompoundIndex::LengthMinus(lm) => {
                    for key in &mut result {
                        let keys = match storage.get(*key).unwrap() {
                            Term::Compound(c) => c.keys(),
                            _ => panic!("attempt to index into non-compound term"),
                        };
                        *key = keys[keys.len() - lm];
                    }
                }
                CompoundIndex::Middle(m) => {
                    let start = m.starting_at_zero_plus();
                    let end = m.ending_at_length_minus();
                    for key in &mut result {
                        let keys = match storage.get(*key).unwrap() {
                            Term::Compound(c) => c.keys(),
                            _ => panic!("attempt to index into non-compound term"),
                        };
                        for n in start..=end {
                            if n >= keys.len() {
                                break;
                            }
                            middle_buffer.push(keys[n]);
                        }
                    }
                    result.clear();
                    result.extend(middle_buffer.drain(..));
                }
            }
        }
        result
    }
}

#[derive(Debug)]
pub struct TermIndex1 {
    initial: Vec<usize>,
    last: MiddleIndices,
}

impl TermIndex1 {
    pub fn new(initial: Vec<usize>, last: MiddleIndices) -> Self {
        Self { initial, last }
    }
}

impl TermIndex1 {
    pub fn count_repetitions(&self, storage: &Storage, mut k: StorageKey) -> usize {
        for &index in &self.initial {
            k = storage.get_compound(k).unwrap().keys()[index];
        }
        let term = storage.get_compound(k).unwrap();
        let length = term.keys().len();
        let zp = self.last.starting_at_zero_plus();
        let lm = self.last.ending_at_length_minus();
        if lm > length {
            0
        } else {
            // We calculate here the size of the following set of integers:
            // size { forall x. x >= first_idx && x <= len - lm } = ???
            // For integers A and B, how many integers are in the range A <= x <= B?
            // If A > B, then 0
            // Otherwise, B-A+1
            let a = zp;
            let b = length - lm;
            b.saturating_sub(a)
                .checked_add(1)
                .expect("overflow when computing repetition count")
        }
    }
}

pub struct TermIndex {
    indices: Vec<usize>,
}

impl TermIndex {
    pub fn new(indices: Vec<usize>) -> Self {
        Self { indices }
    }

    pub fn lookup(&self, storage: &Storage, mut k: StorageKey) -> StorageKey {
        for &index in &self.indices {
            k = storage.get_compound(k).unwrap().keys()[index];
        }
        k
    }
}

#[cfg(test)]
mod test {
    use crate::parser::read;

    use super::*;

    #[test]
    fn count_repetitions1() {
        let mut storage = Storage::new();
        let k = read(&mut storage, "(0 1 2 3 4)".into()).unwrap();
        let index = TermIndex1::new(vec![], MiddleIndices::new(1, 1));
        let repetitions = index.count_repetitions(&storage, k);
        assert_eq!(repetitions, 4);
    }

    #[test]
    fn count_repetitions2() {
        let mut storage = Storage::new();
        let k = read(&mut storage, "(0 (a b c d e (x y) g) 2 3 4)".into()).unwrap();
        let index = TermIndex1::new(vec![1, 5], MiddleIndices::new(0, 1));
        let repetitions = index.count_repetitions(&storage, k);
        assert_eq!(repetitions, 2);
    }
}









// fn get_with_offsets(
//     storage: &Storage,
//     mut k: StorageKey,
//     indices: &[Index],
//     offsets: &[usize],
// ) -> StorageKey {
//     let mut offsets = offsets.iter().copied();
//     for index in indices {
//         match storage.get(k).unwrap() {
//             Term::Compound(c) => {
//                 let zp = match index {
//                     Index::ZeroPlus(zp) => *zp,
//                     Index::LengthMinus(lm) => c.keys().len() - lm,
//                     Index::Middle(m) => {
//                         let offset = offsets.next().expect("no next offset");
//                         m.starting_at_zero_plus() + offset
//                     }
//                 };
//                 k = c.keys()[zp];
//             }
//             _ => panic!("attempt to index into non-compound term"),
//         }
//     }
//     assert!(offsets.next().is_none());
//     k
// }