use std::ptr::NonNull;
use std::mem;
use std::alloc::Layout;
use crate::{alignment, chunks::MemoryChunk, kernel_mem, list::{Header, Node, FreeListNode}};

pub const BLOCKHEADER: usize  = mem::size_of::<Header<Block>>();
pub const MIN_BLOCK_SIZE: usize = mem::size_of::<Node<()>>();
#[derive(Debug)]
pub struct Block {
   pub memory_chunk: NonNull<Header<MemoryChunk>>,

   pub size: usize,

   pub is_free: bool,
}

impl Header<Block> {
     pub unsafe fn from_free_node(node: NonNull<FreeListNode>) -> NonNull<Self> {
       Self::from_ptr(node.cast())
     }

     pub unsafe fn from_aligned_address(address: NonNull<u8>) -> NonNull<Self> {
      *alignment::header_ptr_of(address).as_ptr()
     }

     pub unsafe fn from_allocated_pointer(address: NonNull<u8>, layout: Layout) -> NonNull<Self> {
       if layout.align() <= kernel_mem::POINTER_SIZE {
           Self::from_ptr(address)
       } else {
           Self::from_aligned_address(address)
       }
     }

     pub unsafe fn mut_memorychunk(&mut self) -> &mut Header<MemoryChunk> {
         self.data.memory_chunk.as_mut()
     }

     pub fn let_free(&mut self){
         self.data.is_free = true;
     }
     pub fn not_free(&mut self) {
         self.data.is_free = false;
     }

     pub fn is_free(&self) -> bool {
        self.data.is_free
     }

    pub fn size(&self) -> usize {
        self.data.size
    }

    pub fn total_size(&self) -> usize {
        BLOCKHEADER + self.data.size
    }
}