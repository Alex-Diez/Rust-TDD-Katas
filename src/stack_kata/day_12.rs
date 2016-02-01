use alloc::raw_vec::RawVec;

use std::ops::{Deref, DerefMut};
use std::{slice, ptr};
use std::intrinsics::assume;

pub struct Stack<T> {
    size: usize,
    raw_vec: RawVec<T>
}

impl <T> Deref for Stack<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        unsafe {
            let ptr = self.raw_vec.ptr();
            assume(!ptr.is_null());
            slice::from_raw_parts(ptr, self.size)
        }
    }
}

impl <T> DerefMut for Stack<T> {
    
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe {
            let ptr = self.raw_vec.ptr();
            assume(!ptr.is_null());
            slice::from_raw_parts_mut(ptr, self.size)
        }
    }
}

impl <T> Stack<T> {
    
    pub fn new(max_size: usize) -> Stack<T> {
        Stack { size: 0, raw_vec: RawVec::with_capacity(max_size) }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn push(&mut self, val: T) {
        if self.size < self.raw_vec.cap() {
            unsafe {
                let end = self.as_mut_ptr().offset(self.size as isize);
                ptr::write(end, val);
            }
            self.size += 1;
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if !self.is_empty() {
            self.size -= 1;
            unsafe {
                Some(ptr::read(self.get_unchecked(self.size)))
            }
        }
        else {
            None
        }
    }
}