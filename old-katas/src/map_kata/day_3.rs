#![allow(new_without_default)]

pub struct HashMap {
	size: usize
}

#[allow(dead_code, unused_variables)]
impl HashMap {

    pub fn new(buckets: usize) -> HashMap {
    	HashMap { size: 0 }
    }

    pub fn is_empty(&self) -> bool {
    	self.size() == 0
    }

    pub fn size(&self) -> usize {
    	self.size
    }

    pub fn put(&mut self, key: i32, val: i32) {
    	self.size += 1;
    }

    pub fn contains(&self, key: i32) -> bool {
    	false
    }
}
