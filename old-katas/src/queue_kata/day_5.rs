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

#[derive(Default)]
pub struct Queue {
    size: usize,
    head: Option<Box<Node>>,
    tail: Option<Shared<Node>>
}

#[allow(boxed_local)]
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
                while (*node).elem != e && (*node).next.is_some() {
                    node = (*node).next.as_ref().unwrap();
                }
                (*node).elem == e
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
