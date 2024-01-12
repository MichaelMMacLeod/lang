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
pub struct TermIndex {
    compound_indices: Vec<CompoundIndex>,
}

impl TermIndex {
    pub fn new(indices: Vec<CompoundIndex>) -> Self {
        Self {
            compound_indices: indices,
        }
    }

    pub fn compound_indices(&self) -> &[CompoundIndex] {
        &self.compound_indices
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

pub struct TermIndex2 {
    initial: Vec<usize>,
    last: MiddleIndices,
}

impl TermIndex2 {
    pub fn new(initial: Vec<usize>, last: MiddleIndices) -> Self {
        Self { initial, last }
    }
}

impl TermIndex2 {
    fn count_repetitions(&self, storage: &Storage, mut k: StorageKey) -> usize {
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

#[cfg(test)]
mod test {
    use crate::parser::read;

    use super::*;

    #[test]
    fn count_repetitions1() {
        let mut storage = Storage::new();
        let k = read(&mut storage, "(0 1 2 3 4)".into()).unwrap();
        let index = TermIndex2::new(vec![], MiddleIndices::new(1, 1));
        let repetitions = index.count_repetitions(&storage, k);
        assert_eq!(repetitions, 4);
    }

    #[test]
    fn count_repetitions2() {
        let mut storage = Storage::new();
        let k = read(&mut storage, "(0 (a b c d e (x y) g) 2 3 4)".into()).unwrap();
        let index = TermIndex2::new(vec![1, 5], MiddleIndices::new(0, 1));
        let repetitions = index.count_repetitions(&storage, k);
        assert_eq!(repetitions, 2);
    }
}
