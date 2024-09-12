use std::mem;

use crate::datablock::*;
use crate::kernel_mem;
use crate::kernel_mem::*;
use crate::list::*;
use std::alloc::Layout;
use std::mem::ManuallyDrop;
use std::ptr::NonNull;

pub const REGION_HEADER_SIZE: usize   = mem::size_of::<Header<MemoryChunk>>();
#[derive(Debug)]
pub struct MemoryChunk {
    pub size: usize,
    pub nodes: LinkedList<Block>
}


impl Header<MemoryChunk> {
    pub unsafe fn first(&self) -> NonNull<Node<Block>> {
        self.data.nodes.head.unwrap_unchecked()
    }

    pub fn size(&self) -> usize {
        self.data.size
    }

    pub fn total_size(&self) -> usize {
        let header_size = mem::size_of::<Header<MemoryChunk>>();
        header_size + self.size()
    }
}


pub fn region_len_16(size: usize) -> usize {
    assert_eq!(size, 16); // due it being the smallest size
    let total_size = REGION_HEADER_SIZE + 290 * (BLOCKHEADER + size);
    let mut length = Layout::from_size_align(total_size, unsafe { kernel_mem::Platform::page_size()})
        .unwrap()
        .pad_to_align()
        .size();
    length
}

pub fn region_len_32(size: usize) -> usize {
    let total_size = REGION_HEADER_SIZE + 226 * (BLOCKHEADER + size);
    let mut length = Layout::from_size_align(total_size, unsafe { kernel_mem::Platform::page_size()})
        .unwrap()
        .pad_to_align()
        .size();
    length
}


pub fn region_len_64(size: usize) -> usize {
    let total_size = REGION_HEADER_SIZE + 157 * (BLOCKHEADER + size);
    let mut length = Layout::from_size_align(total_size, unsafe { kernel_mem::Platform::page_size()})
        .unwrap()
        .pad_to_align()
        .size();
    length
}


pub fn region_len_256(size: usize) -> usize {
    let total_size = REGION_HEADER_SIZE + 55 * (BLOCKHEADER + size);
    let mut length = Layout::from_size_align(total_size, unsafe { kernel_mem::Platform::page_size()})
        .unwrap()
        .pad_to_align()
        .size();
    length
}

pub fn region_len_512(size: usize) -> usize {
    let total_size = REGION_HEADER_SIZE + 28 * (BLOCKHEADER + size);
    let mut length = Layout::from_size_align(total_size, unsafe { kernel_mem::Platform::page_size()})
        .unwrap()
        .pad_to_align()
        .size();
    length
}

pub fn region_len_1024(size: usize) -> usize {
    let total_size = REGION_HEADER_SIZE + 14 * (BLOCKHEADER + size);
    let mut length = Layout::from_size_align(total_size, unsafe { kernel_mem::Platform::page_size()})
        .unwrap()
        .pad_to_align()
        .size();
    length
}