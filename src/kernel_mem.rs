
use std::ptr::{self, NonNull};
use libc;
use std::mem;
pub type Pointer<T> = Option<NonNull<T>>;



pub const POINTER_SIZE: usize = mem::size_of::<usize>();
pub trait Memory {
    unsafe fn get_mem(length: usize) -> Pointer<u8>;
    unsafe fn return_mem(address: NonNull<u8>, length: usize);
    unsafe fn page_size() -> usize;
}


pub struct Platform;

// Implemented from https://github.com/antoniosarosi/rulloc/blob/master/src/platform.rs
// I have tried to simplify it for unix only!!!!

impl Memory for Platform {
    unsafe fn get_mem(length: usize) -> Pointer<u8> {
        let protect = libc::PROT_READ | libc::PROT_WRITE;

        let flag = libc::MAP_PRIVATE | libc::MAP_ANONYMOUS;

        match libc::mmap(ptr::null_mut(), length, protect, flag, -1, 0) {
            libc::MAP_FAILED => None,
            address => Some(NonNull::new_unchecked(address).cast()),
        }
   }

    unsafe fn return_mem(address: NonNull<u8>, length: usize) {
       if libc::munmap(address.cast().as_ptr(), length) != 0 {
         todo!()
       }
   }


   unsafe fn page_size() -> usize {
     libc::sysconf(libc::_SC_PAGE_SIZE) as usize
   }
}





