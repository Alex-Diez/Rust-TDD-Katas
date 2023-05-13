type Link = Option<Box<Node>>;

struct Node {
    item: i32,
    next: Link,
}

impl Node {
    fn new(item: i32, next: Link) -> Link {
        Some(Box::new(Node { item, next }))
    }
}

#[derive(Default)]
pub struct Stack {
    head: Link,
}

impl Stack {
    pub(crate) fn push(&mut self, item: i32) {
        self.head = Node::new(item, self.head.take());
    }

    pub(crate) fn pop(&mut self) -> Option<i32> {
        self.head.take().map(|mut node| {
            self.head = node.next.take();
            node.item
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pop_from_empty_stack() {
        let mut stack = Stack::default();

        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn push_single_item() {
        let mut stack = Stack::default();

        stack.push(1);

        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn push_multiple_items() {
        let mut stack = Stack::default();

        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }
}
