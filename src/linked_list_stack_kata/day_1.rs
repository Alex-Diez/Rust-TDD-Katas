use std::mem;

use std::default::Default;

enum Link {
    Empty,
    Cons(Box<Node>)
}

impl Default for Link {

    fn default() -> Link {
        Link::Empty
    }
}

struct Node {
    val: i32,
    next: Link
}

#[derive(Default)]
pub struct Stack {
    head: Link
}

impl Stack {
    pub fn new() -> Stack {
        Stack { head: Default::default() }
    }

    pub fn push(&mut self, e: i32) {
        let new_node = Box::new(Node {
            val: e,
            next: mem::replace(&mut self.head, Default::default())
        });

        self.head = Link::Cons(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::Cons(node) => {
                let node = *node;
                self.head = node.next;
                Some(node.val)
            }
        }
    }
}

impl Drop for Stack {

    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        while let Link::Cons(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}
