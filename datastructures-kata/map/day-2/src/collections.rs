pub struct Map {
    len: usize
}

impl Map {
    
    pub fn new(size: usize) -> Map {
        Map { len: 0 }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn put(&mut self, key: i32, value: i32) {
        self.len += 1; 
    }

    pub fn remove(&mut self, key: i32) {
        self.len -= 1;
    }
}