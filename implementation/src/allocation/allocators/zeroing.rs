use std::alloc::Layout;

use crate::allocation::blocks::map_dynamic_block::MapDynamicBlock;

use super::allocator::Allocator;

pub unsafe fn allocate_zeroed<B: MapDynamicBlock, A: Allocator<B>>(
    a: A,
    layout: Layout,
) -> Result<B, A::AllocateError> {
    let block = a.allocate(layout)?;
    Ok(block.map(|d| d.initialize_with_constant(0)))
}
