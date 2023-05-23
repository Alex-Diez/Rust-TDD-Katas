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
    head: Link<T>,
}

impl<T> Stack<T> {
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.item
        })
    }

    pub fn push(&mut self, item: T) {
        self.head = Node::new(item, self.head.take());
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
        assert_eq!(stack.pop(), None)
    }

    #[test]
    fn push_many_items() {
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
