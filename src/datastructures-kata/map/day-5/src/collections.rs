#![feature(plugin)]
#![plugin(clippy)]

use std::mem;
use std::option::Option;
use std::boxed::Box;
use std::ops::Deref;
use std::ops::DerefMut;

struct Bucket {
    key: Option<i32>,
    next: Option<BucketLink>
}

impl Bucket {
    
    fn new(key: i32) -> Bucket {
        Bucket {
            key: Some(key),
            next: None
        }
    }

    fn empty() -> Bucket {
        Bucket {
            key: None,
            next: None
        }
    }
}

struct BucketLink {
    ptr: *mut Bucket
}

impl BucketLink {

    fn new(bucket: Bucket) -> BucketLink {
        BucketLink {
            ptr: Box::into_raw(Box::new(bucket))
        }
    }
}

impl Deref for BucketLink {
    type Target = Bucket;

    fn deref(&self) -> &Bucket {
        unsafe { mem::transmute(self.ptr) }
    }
}

impl DerefMut for BucketLink {

    fn deref_mut(&mut self) -> &mut Bucket {
        unsafe { mem::transmute(self.ptr) }
    }
}

impl Copy for BucketLink { }

impl Clone for BucketLink {

    fn clone(&self) -> BucketLink {
        BucketLink {
            ptr: self.ptr
        }
    }
}

pub struct Map {
    size: usize,
    table: Vec<BucketLink>
}

impl Map {

    pub fn new() -> Map {
        let mut table = Vec::with_capacity(16);
        for _ in 0..16 {
            table.push(BucketLink::new(Bucket::empty()))
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
        let index = self.table.capacity() & key as usize;
        if !self.contains(key) {
            self.size += 1;
            let mut new_bucket = BucketLink::new(Bucket::new(key));
            let link = self.table.get(index).cloned();
            new_bucket.next = link;
            self.table[index] = new_bucket;
        }
    }

    pub fn contains(&self, key: i32) -> bool {
        let index = self.table.capacity() & key as usize;
        let mut link = match self.table.get(index) {
            Some(&l) => l,
            None => return false,
        };
        while (*link).key != Some(key) && (*link).next.is_some() {
            link = (*link).next.unwrap();
        }
        (*link).key == Some(key)
    }
}
