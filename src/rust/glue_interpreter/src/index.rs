use crate::storage::{Storage, StorageKey, Term};

pub struct Index6s {
    indices: Vec<Index6>,
}

impl Index6s {
    pub fn new(indices: Vec<Index6>) -> Self {
        Self { indices }
    }
}

pub enum Index6 {
    ZeroPlus(ZeroPlus),
    LenMinus(LenMinus),
    Middle(Middle),
}

impl Index6 {
    pub fn zero_plus(n: usize) -> Self {
        Self::ZeroPlus(ZeroPlus(n))
    }

    pub fn len_minus(n: usize) -> Self {
        Self::LenMinus(LenMinus(n))
    }

    pub fn middle(zero_plus: usize, len_minus: usize) -> Self {
        Self::Middle(Middle {
            zero_plus: ZeroPlus(zero_plus),
            len_minus: LenMinus(len_minus),
            current: ZeroPlus(zero_plus),
        })
    }
}

pub struct ZeroPlus(usize);

pub struct LenMinus(usize);

pub struct Middle {
    zero_plus: ZeroPlus,
    len_minus: LenMinus,
    current: ZeroPlus,
}

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

    pub fn count_repetitions(&self, storage: &Storage, k: StorageKey) -> usize {
        let term = storage.get_compound(k).unwrap();
        let length = term.keys().len();
        let zp = self.starting_at_zero_plus();
        let lm = self.ending_at_length_minus();
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

    pub fn to_index5(self) -> Index5 {
        let mut nomiddle_stack: Vec<Nomiddle> = vec![];
        let mut index4_stack: Vec<Index4> = vec![];

        for index in self.compound_indices {
            match index {
                CompoundIndex::ZeroPlus(zp) => {
                    nomiddle_stack.push(Nomiddle::ZeroPlus(zp));
                }
                CompoundIndex::LengthMinus(lm) => {
                    nomiddle_stack.push(Nomiddle::LenMinus(lm));
                }
                CompoundIndex::Middle(m) => index4_stack.push(Index4 {
                    first: nomiddle_stack.drain(..).collect(),
                    last: m,
                }),
            }
        }

        match index4_stack.len() {
            0 => Index5::WithoutMiddle(nomiddle_stack),
            _ => Index5::WithMiddle(index4_stack),
        }
    }

    pub fn empty() -> Self {
        Self::new(vec![])
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

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub enum Nomiddle {
    ZeroPlus(usize),
    LenMinus(usize),
}

#[derive(Clone, Debug)]
pub struct NomiddleIndex {
    indices: Vec<Nomiddle>,
}

pub fn zp_lookup(indices: &[usize], storage: &Storage, mut k: StorageKey) -> StorageKey {
    for &index in indices {
        k = storage.get_compound(k).unwrap().keys()[index];
    }
    k
}

impl NomiddleIndex {
    pub fn new(indices: Vec<Nomiddle>) -> Self {
        Self { indices }
    }

    pub fn lookup(self, storage: &Storage, mut k: StorageKey) -> StorageKey {
        for index in self.indices {
            let zp = match index {
                Nomiddle::ZeroPlus(zp) => zp,
                Nomiddle::LenMinus(lm) => {
                    let len = storage.get_compound(k).unwrap().keys().len();
                    len.checked_sub(lm).unwrap()
                }
            };
            k = storage.get_compound(k).unwrap().keys()[zp];
        }

        k
    }

    pub fn lookup2(self, storage: &Storage, mut k: StorageKey, offsets: &[usize]) -> StorageKey {
        for &offset in offsets {
            k = storage.get_compound(k).unwrap().keys()[offset];
        }

        for index in self.indices {
            let zp = match index {
                Nomiddle::ZeroPlus(zp) => zp,
                Nomiddle::LenMinus(lm) => {
                    let len = storage.get_compound(k).unwrap().keys().len();
                    len.checked_sub(lm).unwrap()
                }
            };
            k = storage.get_compound(k).unwrap().keys()[zp];
        }

        k
    }

    pub fn to_zp(self, storage: &Storage, mut k: StorageKey, offsets: &[usize]) -> Vec<usize> {
        for &offset in offsets {
            k = storage.get_compound(k).unwrap().keys()[offset];
        }

        self.indices
            .into_iter()
            .map(|nomiddle_index| {
                let zp = match nomiddle_index {
                    Nomiddle::ZeroPlus(zp) => zp,
                    Nomiddle::LenMinus(lm) => {
                        let len = storage.get_compound(k).unwrap().keys().len();
                        len.checked_sub(lm).unwrap()
                    }
                };
                k = storage.get_compound(k).unwrap().keys()[zp];
                zp
            })
            .collect()
    }
}

#[derive(Clone, Debug)]
pub struct Index4 {
    pub first: Vec<Nomiddle>,
    pub last: MiddleIndices,
}

impl Index4 {
    pub fn new(first: Vec<Nomiddle>, last: MiddleIndices) -> Self {
        Self { first, last }
    }
}

#[derive(Clone, Debug)]
pub enum Index5 {
    WithMiddle(Vec<Index4>),
    WithoutMiddle(Vec<Nomiddle>),
}

impl Index5 {
    pub fn new_with_middle(indices: Vec<Index4>) -> Self {
        Self::WithMiddle(indices)
    }

    pub fn new_without_middle(indices: Vec<Nomiddle>) -> Self {
        Self::WithoutMiddle(indices)
    }

    pub fn empty() -> Self {
        Self::new_without_middle(vec![])
    }

    pub fn prepend(&mut self, offsets: &[usize]) {
        match self {
            Index5::WithMiddle(m) => {
                if let Some(f) = m.first_mut() {
                    let new_elements: Vec<Nomiddle> = offsets
                        .iter()
                        .map(|offset| Nomiddle::ZeroPlus(*offset))
                        .chain(f.first.drain(..))
                        .collect();
                    f.first = new_elements;
                } else {
                    *self = Index5::WithoutMiddle(
                        offsets
                            .iter()
                            .map(|offset| Nomiddle::ZeroPlus(*offset))
                            .collect(),
                    )
                }
            }
            Index5::WithoutMiddle(m) => {
                let new_elements: Vec<Nomiddle> = offsets
                    .iter()
                    .map(|offset| Nomiddle::ZeroPlus(*offset))
                    .chain(m.drain(..))
                    .collect();
                *m = new_elements;
            }
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
