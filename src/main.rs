#![feature(strict_provenance)]
#![feature(slice_ptr_get)]
pub mod datablock;
pub mod kernel_mem;
pub mod chunks;
pub mod list;
pub mod alignment;
pub mod allocator;

use std::{alloc::{GlobalAlloc, Layout}, io::Write, ptr::{self, NonNull}, sync::Mutex};

use crate::allocator::{Single_Allocator, Reallocator};


fn main() {
   
}
