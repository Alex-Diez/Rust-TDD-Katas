use std::option::Option;

pub struct Map {
    size: usize,
    buckets: Vec<i32>
}

impl Map {
    
    pub fn new(buckets: usize) -> Map {
        Map { size: 0, buckets: Vec::with_capacity(buckets) }
    }

    pub fn is_empty(&self) -> bool {
        self.size() == 0
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn put(&mut self, key: i32, val: i32) {
        if(!self.contains(key)) {
            self.size += 1;
            self.buckets.push(key);
        }
    }

    pub fn contains(&self, key: i32) -> bool {
        self.buckets.contains(&key)
    }

    pub fn get(&self, key: i32) -> Option<i32> {
        None
    }
}
