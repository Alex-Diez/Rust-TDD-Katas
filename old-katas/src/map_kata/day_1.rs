#[derive(Default)]
pub struct Map {
    elems: Vec<i32>
}

#[allow(dead_code, unused_variables)]
impl Map {

    pub fn new() -> Map {
        Map { elems: Vec::new() }
    }

    pub fn size(&self) -> usize {
        self.elems.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elems.is_empty()
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if !self.elems.contains(&key) {
            self.elems.push(key);
        }
    }

    pub fn remove(&mut self, key: i32) {
        self.elems.pop();
    }
}
