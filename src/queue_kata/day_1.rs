use std::ptr;
use std::boxed::Box;
use std::option::Option;

type Link = Option<Box<Node>>;

struct Node {
    elem: i32,
    next: Link
}

pub struct Queue {
    size: usize,
    head: Link,
    tail: *mut Node
}

impl Queue {

    pub fn new() -> Queue {
        Queue {
            size: 0,
            head: None,
            tail: ptr::null_mut()
        }
    }

    pub fn is_empty(&self) -> bool {
        self.size() == 0
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn enqueue(&mut self, e: i32) {
        self.size += 1;
        let mut new_tail = Box::new(Node { elem: e, next: None });
        let raw_tail: *mut _ = &mut *new_tail;
        if !self.tail.is_null() {
            unsafe {
                (*self.tail).next = Some(new_tail);
            }
        }
        else {
            self.head = Some(new_tail);
        }
        self.tail = raw_tail;
    }

    pub fn dequeue(&mut self) {
        self.size -= 1;
    }

    pub fn contains(&self, val: i32) -> bool {
        let ref node = self.head;
        let mut find = false;
 /*       loop {
            match node {
                Some(n) => {
                    find = n.elem == val;
                    node = n.next;
                },
                None => break,
            }
        }*/
        find
    }
}