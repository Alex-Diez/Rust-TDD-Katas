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

#[cfg(test)]
pub mod tests {
    use super::Stack;

    #[test]
    fn create_an_empty_stack() {
        let mut stack = Stack::new();
        assert_eq!(stack.pop(), None)
    }

    #[test]
    fn test_add_element_to_a_stack() {
        let mut stack = Stack::new();
        stack.push(1);
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_add_three_elements_to_a_stack() {
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
