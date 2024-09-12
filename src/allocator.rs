use crate::{list::*,alignment, chunks::{self, *, MemoryChunk, REGION_HEADER_SIZE}, datablock::*, kernel_mem::{self, Memory}, list::{self, FreeList, Header}};
use crate::kernel_mem::Pointer;
use std::{alloc::Layout, fmt::Error, mem::ManuallyDrop, ptr::{self, NonNull}};

#[derive(Debug)]
pub struct Single_Allocator {
   free_blocks_16: ManuallyDrop<FreeList>,
   free_blocks_32: ManuallyDrop<FreeList>,
   free_blocks_64: ManuallyDrop<FreeList>,
   free_blocks_256: ManuallyDrop<FreeList>,
   free_blocks_512: ManuallyDrop<FreeList>,
   free_blocks_1024: ManuallyDrop<FreeList>,
   chunks: ManuallyDrop<LinkedList<MemoryChunk>>,
}

pub struct Reallocator {
    pub block: NonNull<Header<Block>>,
    pub addr: NonNull<u8>,
    pub new_layout: Layout,
    pub old_layout: Layout,
}

impl Reallocator {
     pub unsafe fn new(
        addr: NonNull<u8>,
        new: Layout,
        old: Layout
     ) -> Self {
        let block = Header::from_allocated_pointer(addr, old);

        Self { block: block, addr: addr, new_layout: new, old_layout: old }
     }

     pub fn size(&self) -> usize {
        if self.old_layout.size() < self.new_layout.size() {
            return self.new_layout.size()
        } else {
            return self.old_layout.size()
        }
     }
}

impl Single_Allocator {

    pub const fn new() -> Self {
        Self {
            free_blocks_16: ManuallyDrop::new(FreeList::new()),
            free_blocks_32: ManuallyDrop::new(FreeList::new()),
            free_blocks_64: ManuallyDrop::new(FreeList::new()),
            free_blocks_256: ManuallyDrop::new(FreeList::new()),
            free_blocks_512: ManuallyDrop::new(FreeList::new()),
            free_blocks_1024: ManuallyDrop::new(FreeList::new()),
            chunks: ManuallyDrop::new(LinkedList::new()),
        }
    }



    pub unsafe fn allocate(&mut self, layout: Layout) -> NonNull<[u8]> { 
        let size = alignment::minimum_block_size_for(layout);
        let free_block = match self.find_block(size) {
            Some(block) => block, // Test code for now
            None => self.get_memory_chunk(size).as_ref().first()
        };
        
        match size {
            0..=16 => self.free_blocks_16.remove_block(free_block),
            17..=32 => self.free_blocks_32.remove_block(free_block),
            33..=64 => self.free_blocks_64.remove_block(free_block),
            65..=256 => self.free_blocks_256.remove_block(free_block),
            257..=512 => self.free_blocks_512.remove_block(free_block),
            513..=1024 => self.free_blocks_1024.remove_block(free_block),
            _ => unreachable!()
        }

        let address = self.add_padding(free_block, layout.align());
         
        address
    }
    
    pub unsafe fn deallocate(&mut self, address: NonNull<u8>, layout: Layout) {
        let mut block = Header::<Block>::from_allocated_pointer(address, layout);
        let size = layout.size();

        match size {
            0..=16 => self.free_blocks_16.append_block(block),
            17..=32 => self.free_blocks_32.append_block(block),
            33..=64 => self.free_blocks_64.append_block(block),
            65..=256 => self.free_blocks_256.append_block(block),
            257..=512 => self.free_blocks_512.append_block(block),
            513..=1024 => self.free_blocks_1024.append_block(block),
            _ => unreachable!()
        }
    }
    

    pub unsafe fn reallocate(&mut self, realloc: &Reallocator) -> NonNull<[u8]>{
        let result = self.realloc_move(realloc);
         result.unwrap()
    }
    unsafe fn realloc_move(&mut self, realloc: &Reallocator) -> Result<NonNull<[u8]>, Error>{
        let new_addr = self.allocate(realloc.new_layout);
        ptr::copy_nonoverlapping(realloc.addr.as_ptr(), new_addr.as_mut_ptr(), realloc.size());

        self.deallocate(realloc.addr, realloc.old_layout);

        Ok(new_addr)
    }


    unsafe fn find_block(&self, size: usize) -> Pointer<Header<Block>> {
        match size {
            0..=16 => {
                self.free_blocks_16.iter()
                .map(|node| Header::<Block>::from_free_node(node)).next()
            },
            17..=32 => {
                self.free_blocks_32.iter()
                .map(|node| Header::<Block>::from_free_node(node)).next()
            },
            33..=64 => {
                self.free_blocks_64.iter()
                .map(|node| Header::<Block>::from_free_node(node)).next()
            },
            65..=256 => {
                self.free_blocks_256.iter()
                .map(|node| Header::<Block>::from_free_node(node)).next()
            },
            257..=512 => {
                self.free_blocks_256.iter()
                .map(|node| Header::<Block>::from_free_node(node)).next()
            },
            513..=1024 => {
                self.free_blocks_256.iter()
                .map(|node| Header::<Block>::from_free_node(node)).next()
            },
            _ => panic!("No size bigger than 1024")
        }
    }
    
    unsafe fn add_padding(&self , block: NonNull<Header<Block>>, align: usize) -> NonNull<[u8]> {
        let content = NonNull::new_unchecked(block.as_ptr().offset(1)).cast();
        if align < kernel_mem::POINTER_SIZE {
            return NonNull::slice_from_raw_parts(content, block.as_ref().size());
        }

        let (next_align, padding) = alignment::next_aligned(content, align);

        ptr::write(alignment::header_ptr_of(next_align).as_ptr(), block);
    
        return NonNull::slice_from_raw_parts(next_align, block.as_ref().size() - padding)
    }

    unsafe fn get_memory_chunk(&mut self, size: usize) -> NonNull<Header<MemoryChunk>> {
        let mut length = 0;
        let mut chunk_size = 0;
        if size <= 16 {
           length = region_len_16(size);
           chunk_size = 16;
        } else if size <= 32 {
            length = region_len_32(size);
            chunk_size = 32;
        } else if size <= 64 {
            length = region_len_64(size);
            chunk_size = 64;
        }
        else if size <= 256 {
            length = region_len_256(size);
            chunk_size = 256;
        }
        else if size <= 512 {
            length = region_len_512(size);
            chunk_size = 512;
        }
        else if size <= 1024 {
            length = region_len_1024(size);
            chunk_size = 1024;
        } else {
            unimplemented!()
        }
        let address = kernel_mem::Platform::get_mem(length).unwrap();
        let mut memorychunk = self.chunks.push(
            MemoryChunk {
                nodes: LinkedList::new(),
                size: length - REGION_HEADER_SIZE 
            },
            address
        );
        
        let block_size = size + BLOCKHEADER; // Total size of each block including the header
        let mut current_address = NonNull::new_unchecked(memorychunk.as_ptr().offset(1)).cast::<u8>(); // Start of the memory region
    
        let mut remaining_size = memorychunk.as_ref().size(); // Remaining size to be divided into blocks

        while remaining_size >= REGION_HEADER_SIZE {
            
            let block = memorychunk.as_mut().data.nodes.push(
                Block {
                size: block_size - BLOCKHEADER, 
                is_free: true,
                memory_chunk: memorychunk
            },
            current_address
            );

            // Add the block to the free list
            let mut block_header = match chunk_size {
                16 => {
                    self.free_blocks_16.append_block(block);
                },
                32 => {
                    self.free_blocks_32.append_block(block);
                },
                64 => {
                    self.free_blocks_64.append_block(block);
                },
                256 => {
                    self.free_blocks_256.append_block(block);
                },
                512 => {
                    self.free_blocks_512.append_block(block);
                },
                1024 => {
                    self.free_blocks_1024.append_block(block);
                },
                _ => unreachable!()
            };
            // Move the current address to the next block
            current_address = NonNull::new_unchecked(current_address.as_ptr().add(block_size));
            // Decrease the remaining size
            
            // Exception 
            if remaining_size <= block_size {
                break
            }
            remaining_size -= block_size;
        };
        // Return the memory chunk
        memorychunk
    }
    
}
