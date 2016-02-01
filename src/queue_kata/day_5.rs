use std::boxed::Box;
use std::ptr::Shared;
use std::option::Option;

struct Node {
    elem: i32,
    next: Option<Box<Node>>
}

impl Node {
    
    fn new(e: i32) -> Node {
        Node {
            elem: e,
            next: None
        }
    }
}

pub struct Queue {
    size: usize,
    head: Option<Box<Node>>,
    tail: Option<Shared<Node>>
}

impl Queue {

    pub fn new() -> Queue {
        Queue {
            size: 0,
            head: None,
            tail: None
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn enqueue(&mut self, e: i32) {
        self.size += 1;
        let mut node = Box::new(Node::new(e));
        let raw: *mut _ = &mut *node;
        match self.tail {
            Some(share) => unsafe { (**share).next = Some(node) },
            None => self.head = Some(node),
        }
        unsafe {
            self.tail = Some(Shared::new(raw));
        }
    }

    pub fn contains(&self, e: i32) -> bool {
        match self.head {
            Some(ref head) => {
                let mut node = head;
                let mut find = false;
                loop {
                    if (*node).elem == e {
                        find = true;
                        break; 
                    }
                    match (*node).next {
                        Some(ref next) => node = next,
                        None => break,
                    }
                }
                find
            },
            None => false,
        }
    }

    pub fn dequeue(&mut self) -> Option<i32> {
        self.head.take().map(
            |head| {
                let h = *head;
                self.head = h.next;
                self.size -= 1;
                h.elem
            }
        )
    }
}