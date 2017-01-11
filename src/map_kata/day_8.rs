#![allow(transmute_ptr_to_ref)]

use std::mem;
use std::boxed::Box;
use std::option::Option;
use std::ops::Deref;
use std::ops::DerefMut;
use std::clone::Clone;
use std::marker::Copy;

struct Bucket {
    key: Option<i32>,
    value: Option<i32>,
    next: Option<Link>
}

impl Bucket {

    fn new(key: i32, value: i32) -> Bucket {
        Bucket {
            key: Some(key),
            value: Some(value),
            next: None
        }
    }

    fn empty() -> Bucket {
        Bucket {
            key: None,
            value: None,
            next: None
        }
    }
}

struct Link {
    ptr: *mut Bucket
}

impl Link {

    fn new(bucket: Bucket) -> Link {
        Link {
            ptr: Box::into_raw(Box::new(bucket))
        }
    }
}

impl Deref for Link {
    type Target = Bucket;

    fn deref(&self) -> &Bucket {
        unsafe { mem::transmute(self.ptr) }
    }
}

impl DerefMut for Link {

    fn deref_mut(&mut self) -> &mut Bucket {
        unsafe { mem::transmute(self.ptr) }
    }
}

impl Copy for Link { }
impl Clone for Link {

    fn clone(&self) -> Link {
        Link {
            ptr: self.ptr
        }
    }
}

#[derive(Default)]
pub struct Map {
    size: usize,
    table: Vec<Link>
}

const CAPACITY: usize = 16;

impl Map {

    pub fn new() -> Map {
        let mut table = Vec::with_capacity(CAPACITY);
        for _ in 0..CAPACITY {
            table.push(Link::new(Bucket::empty()));
        }
        Map {
            size: 0,
            table: table
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn insert(&mut self, key: i32, value: i32) {
        let index = (CAPACITY - 1) & key as usize;
        let mut link = self.table[index];
        while (*link).key != Some(key) && (*link).next.is_some() {
            link = (*link).next.unwrap();
        }
        if (*link).key == Some(key) {
            (*link).value = Some(value);
        }
        else {
            self.size += 1;
            let mut new_bucket = Bucket::new(key, value);
            let link = self.table[index];
            new_bucket.next = Some(link);
            self.table[index] = Link::new(new_bucket);
        }
    }

    pub fn contains(&self, key: i32) -> bool {
        let index = (CAPACITY - 1) & key as usize;
        let mut link = self.table[index];
        while (*link).key != Some(key) && (*link).next.is_some() {
            link = (*link).next.unwrap();
        }
        (*link).key == Some(key)
    }

    pub fn get(&self, key: i32) -> Option<i32> {
        let index = (CAPACITY - 1) & key as usize;
        let mut link = self.table[index];
        while (*link).key != Some(key) && (*link).next.is_some() {
            link = (*link).next.unwrap();
        }
        if (*link).key == Some(key) {
            (*link).value
        }
        else {
            None
        }
    }
}
