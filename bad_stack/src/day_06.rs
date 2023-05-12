type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    item: T,
    next: Link<T>,
}

impl<T> Node<T> {
    fn new(item: T, next: Link<T>) -> Link<T> {
        Some(Box::new(Node { item, next }))
    }
}

#[derive(Default)]
pub struct Stack<T> {
    item: Link<T>,
}

impl<T> Stack<T> {
    pub(crate) fn push(&mut self, item: T) {
        self.item = Node::new(item, self.item.take());
    }

    pub(crate) fn pop(&mut self) -> Option<T> {
        self.item.take().map(|mut node| {
            self.item = node.next.take();
            node.item
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pop_from_empty_stack() {
        let mut stack: Stack<i32> = Stack::default();

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
