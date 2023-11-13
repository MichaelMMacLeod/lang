use crate::alignment::Alignment;

pub struct Aligned<V> {
    val: V,
    alignment: Alignment,
}
