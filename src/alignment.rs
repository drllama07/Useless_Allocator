use core::alloc;
use std::alloc::Layout;
use std::f32::MIN;
use std::ptr::NonNull;

use crate::list::*;
use crate::kernel_mem::POINTER_SIZE;
use crate::datablock::{Block, MIN_BLOCK_SIZE};

pub type Header_ptr = NonNull<Header<Block>>;

pub unsafe fn header_ptr_of(address: NonNull<u8>) -> NonNull<Header_ptr> {
    NonNull::new_unchecked(address.cast::<Header_ptr>().as_ptr().offset(-1))
}

fn padding_needed_for(size: usize, align: usize) -> usize {
    (align - (size % align)) % align
}

pub fn minimum_block_size_for(layout: Layout) -> usize {
    let mut size = layout.size() + padding_needed_for(layout.size(), POINTER_SIZE);
    

    if layout.align() > POINTER_SIZE {
        size += layout.align();
    }
    
    if size < MIN_BLOCK_SIZE {
        return MIN_BLOCK_SIZE
    }
    size 
}

pub fn minimum_block_size_no_padding(layout: Layout) -> usize {
    if layout.size() <= MIN_BLOCK_SIZE {
        MIN_BLOCK_SIZE
    } else {
        layout.size() + padding_needed_for(layout.size(), POINTER_SIZE)
    }
}
pub unsafe fn next_aligned(address: NonNull<u8>, align: usize) -> (NonNull<u8>, usize) {
   let align_offset = address.as_ptr().align_offset(align);


   let padding = if align > POINTER_SIZE && align_offset == 0 {
      align 
   } else {
     align_offset
   };

   let next_aligned = address.as_ptr().map_addr(|addr| addr + padding);
   
   (NonNull::new_unchecked(next_aligned), padding)
}

pub unsafe fn padding_required(address: NonNull<u8>, align: usize) -> usize {
    let (_, padding) = next_aligned(address, align);
    padding
}
