use std::ptr::Shared;

struct Node {
    item: i32,
    next: Option<Shared<Node>>
}

impl Node {
    pub fn new(item: i32) -> Option<Shared<Node>> {
        Shared::new(Box::into_raw(Box::new(Node { item: item, next: None })))
    }
}

pub struct Queue {
    head: Option<Shared<Node>>,
    tail: Option<Shared<Node>>
}

impl Queue {
    pub fn new() -> Queue {
        Queue { head: None, tail: None }
    }

    pub fn deque(&mut self) -> Option<i32> {
        self.head.take().map(|mut head| unsafe {
            match head.as_mut().next.take() {
                Some(new_head) => self.head = Some(new_head),
                None => self.tail = None
            }
            head.as_ref().item
        })
    }

    pub fn enqueue(&mut self, item: i32) {
        let node = Node::new(item);
        match self.tail.take() {
            Some(mut tail) => unsafe { tail.as_mut().next = node.clone(); },
            None => self.head = node.clone()
        }
        self.tail = node
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deque_from_empty_queue() {
        let mut queue = Queue::new();

        assert_eq!(queue.deque(), None);
    }

    #[test]
    fn enqueue_one_item() {
        let mut queue = Queue::new();

        queue.enqueue(10);

        assert_eq!(queue.deque(), Some(10));
        assert_eq!(queue.deque(), None);
    }

    #[test]
    fn enqueue_deque_many_items() {
        let mut queue = Queue::new();

        queue.enqueue(10);
        queue.enqueue(20);
        queue.enqueue(30);

        assert_eq!(queue.deque(), Some(10));
        assert_eq!(queue.deque(), Some(20));
        assert_eq!(queue.deque(), Some(30));
        assert_eq!(queue.deque(), None);
    }

    #[test]
    fn enqueue_deque_items_one_by_one() {
        let mut queue = Queue::new();

        queue.enqueue(10);

        assert_eq!(queue.deque(), Some(10));
        assert_eq!(queue.deque(), None);

        queue.enqueue(20);

        assert_eq!(queue.deque(), Some(20));
        assert_eq!(queue.deque(), None);

        queue.enqueue(30);

        assert_eq!(queue.deque(), Some(30));
        assert_eq!(queue.deque(), None);
    }
}
