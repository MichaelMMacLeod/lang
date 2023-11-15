use num_traits::{CheckedMul, PrimInt};

/// Represents a certain number of bytes.
#[derive(Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Bytes<T>(T);

/// Represents a certain number of L1 (data) cache lines.
#[derive(Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct L1CacheLines<T>(T);

/// Represents a certain number of L2 cache lines.
#[derive(Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct L2CacheLines<T>(T);

/// Represents a certain number of L3 cache lines.
#[derive(Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct L3CacheLines<T>(T);

/// Represents a certain number of normal pages.
#[derive(Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct NormalPages<T>(T);

/// Represents a certain number of huge pages.
#[derive(Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct HugePages<T>(T);