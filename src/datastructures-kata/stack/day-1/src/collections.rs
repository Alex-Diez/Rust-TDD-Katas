pub struct Stack<T> {
    max_size: usize,
    elements: Vec<T>
}

impl <T> Stack<T> {
    
    pub fn new(max_size: usize) -> Stack<T> {
        Stack {
            elements: Vec::new(),
            max_size: max_size
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    pub fn push(&mut self, e: T) {
        println!("size - {:?}", self.size());
        println!("max size - {:?}", self.max_size);
        if self.size() == self.max_size {
            panic!("stack over flow");
        }
        self.elements.push(e);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }
}
