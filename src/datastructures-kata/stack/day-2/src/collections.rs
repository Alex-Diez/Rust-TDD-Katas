#![feature(core_intrinsics, alloc)]

extern crate alloc;

use alloc::raw_vec::RawVec;

use std::ops::{Deref, DerefMut};
use std::{slice, ptr};
use std::intrinsics::assume;

pub struct Stack {
    size: usize,
    buf: RawVec<i32>
}

impl Deref for Stack {
    type Target = [i32];
    
    fn deref(&self) -> &[i32] {
        unsafe {
            let p = self.buf.ptr();
            assume(!p.is_null());
            slice::from_raw_parts(p, self.size)
        }
    }
}

impl DerefMut for Stack {

    fn deref_mut(&mut self) -> &mut [i32] {
        unsafe {
            let ptr = self.buf.ptr();
            assume(!ptr.is_null());
            slice::from_raw_parts_mut(ptr, self.size)
        }
    }
}

impl Stack {

    pub fn new(max_size: usize) -> Stack {
        Stack { size: 0, buf: RawVec::with_capacity(max_size) }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn push(&mut self, v: i32) -> bool {
        if self.size == self.buf.cap() {
            false
        }
        else {
            unsafe {
                let end = self.as_mut_ptr().offset(self.size as isize);
                ptr::write(end, v);
                self.size += 1;
            }
            true
        }
    }

    pub fn pop(&mut self) -> Option<i32> {
        if self.is_empty() {
            None
        }
        else {
            unsafe {
                self.size -= 1;
                Some(ptr::read(self.get_unchecked(self.size)))
            }
        }
    }
}