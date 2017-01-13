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
        let node = Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty)
        };

        self.head = Link::Cons(Box::new(node));
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

mod tests {
    use super::Stack;

    #[test]
    fn creates_an_empty() {
        let mut stack: Stack<i32> = Stack::new();

        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn adds_an_element_to_a_stack() {
        let mut stack = Stack::new();

        stack.push(1);

        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn adds_elements_to_a_stack() {
        let mut stack = Stack::new();

        stack.push(10);
        stack.push(20);
        stack.push(30);

        assert_eq!(stack.pop(), Some(30));
        assert_eq!(stack.pop(), Some(20));
        assert_eq!(stack.pop(), Some(10));
        assert_eq!(stack.pop(), None);
    }
}
