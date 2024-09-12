use std::{ptr::NonNull, marker::PhantomData};
use crate::{datablock::Block, kernel_mem::Pointer};


#[derive(Debug)]
pub struct Node<T> {
    pub next: Pointer<Self>,
    pub prev: Pointer<Self>,
    pub data: T
}

// These are for making it work but it is not multithreaded 

unsafe impl<T> Send for Node<T> {}
unsafe impl<T> Sync for Node<T> {}

pub type Header<T> = Node<T>;


impl<T> Header<T> {
    pub unsafe fn from_ptr(ptr: NonNull<u8>) -> NonNull<Self> {
        NonNull::new_unchecked(ptr.as_ptr().cast::<Self>().offset(-1))
    }
}



#[derive(Debug)]
pub struct LinkedList<T> {
    pub head: Pointer<Node<T>>,
    pub tail: Pointer<Node<T>>,
    pub len: usize,
    pub _marker: PhantomData<T>
}

pub struct Iterators<T> {
    current: Pointer<Node<T>>,
    len: usize,
    _marker: PhantomData<T>
}

// A good start point and an example -> 
// Heavily influenced by https://github.com/antoniosarosi/rulloc/blob/master/src/list.rs
impl<T> LinkedList<T> {
    pub const fn new() -> Self {
        Self {
            head: None ,
            tail: None, 
            len: 0,
            _marker: PhantomData
        }
    }

    pub unsafe fn push(&mut self, data: T, address: NonNull<u8>) -> NonNull<Node<T>> {
        let new_node = address.cast::<Node<T>>();
        let mut node = self.tail;

        new_node.as_ptr().write(Node { next: None, prev: node, data: data });
        if let Some(mut tail) = self.tail {
            tail.as_mut().next = Some(new_node)
        } else {
            self.head = Some(new_node)
        }
        self.tail = Some(new_node);

        self.len += 1;
        
        new_node
    }

    pub unsafe fn remove(&mut self, mut node: NonNull<Node<T>>) {
        if self.len == 1 {
            self.head = None;
            self.tail = None;
        } else if node == self.head.unwrap() {
            node.as_mut().next.unwrap().as_mut().prev = None;
            self.head = node.as_ref().next;
        } else if node == self.tail.unwrap() {
            node.as_mut().prev.unwrap().as_mut().next = None;
            self.head = node.as_ref().prev;
        } else {
            let mut next = node.as_ref().next.unwrap();
            let mut prev = node.as_ref().prev.unwrap();

            prev.as_mut().next = Some(next);
            next.as_mut().prev = Some(prev)
        }

        self.len -= 1;
    }

    pub fn iter(&self) -> Iterators<T> {
        Iterators {
            current: self.head,
            len: self.len,
            _marker: PhantomData
        }
    }
}

impl<T> Iterator for Iterators<T> {
    type Item = NonNull<Node<T>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.map(|node| unsafe {
            self.current = node.as_ref().next;
            self.len -= 1;
            node
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<T> IntoIterator for &LinkedList<T> {
    type Item = NonNull<Node<T>>;

    type IntoIter = Iterators<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}


pub type FreeListNode = Node<()>;

pub type FreeList = LinkedList<()>;


impl FreeList {
    pub unsafe fn append_block(&mut self, mut block: NonNull<Header<Block>>) {
        self.push((), NonNull::new_unchecked(block.as_ptr().offset(1)).cast());
        block.as_mut().data.is_free = false;
    }

    pub unsafe fn remove_block(&mut self, mut block: NonNull<Header<Block>>) {
        self.remove(NonNull::new_unchecked(block.as_ptr().offset(1)).cast());
        block.as_mut().data.is_free = true;
    }

    pub unsafe fn iter_blocks(&self) -> impl Iterator<Item = NonNull<Header<Block>>> {
        self.iter()
            .map(|node| Header::<Block>::from_free_node(node))
    }
}