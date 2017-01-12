use std::mem;

use std::default::Default;

struct Node<E> {
    elem: E,
    next: Link<E>
}

enum Link<E> {
    Empty,
    Cons(Box<Node<E>>)
}

impl <E> Default for Link<E> {

    fn default() -> Self {
        Link::Empty
    }
}

#[derive(Default)]
pub struct Stack<E> {
    head: Link<E>
}

impl <E> Stack<E> {

    pub fn new() -> Self {
        Stack { head: Link::Empty }
    }

    pub fn push(&mut self, elem: E) {
        let head = Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty)
        };
        self.head = Link::Cons(Box::new(head));
    }

    pub fn pop(&mut self) -> Option<E> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::Cons(boxed) => {
                let node = *boxed;
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}
