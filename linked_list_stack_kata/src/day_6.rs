use std::mem;

struct Node {
    elem: i32,
    next: Link
}

enum Link {
    Empty,
    Cons(Box<Node>)
}

pub struct Stack {
    head: Link
}

impl Stack {

    pub fn new() -> Self {
        Stack { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let node = Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty)
        });

        self.head = Link::Cons(node);
    }

    pub fn pop(&mut self) -> Option<i32> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_an_empty_stack() {
        let mut stack = Stack::new();

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
