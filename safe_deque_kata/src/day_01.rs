use std::cell::RefCell;
use std::rc::Rc;

type Link = Option<Rc<RefCell<Node>>>;

struct Node {
    item: i32,
    next: Link,
    prev: Link
}

impl Node {
    fn new(item: i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node { item, next: None, prev: None }))
    }
}

pub struct Deque {
    head: Link,
    tail: Link
}

impl Deque {
    pub fn pop_front(&mut self) -> Option<i32> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev.take();
                    self.head = Some(new_head);
                }
                None => {
                    self.tail.take();
                }
            }
            old_head.borrow_mut().item
        })
    }

    pub fn push_front(&mut self, item: i32) {
        let new_head = Node::new(item);
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head);
                self.head = Some(new_head)
            },
            None => {
                self.tail = Some(new_head.clone());
                self.head = Some(new_head);
            }
        }

    }

    pub fn pop_back(&mut self) -> Option<i32> {
        self.tail.take().map(|old_tail| {
            match old_tail.borrow_mut().prev.take() {
                Some(new_tail) => {
                    new_tail.borrow_mut().next.take();
                    self.tail = Some(new_tail);
                },
                None => {
                    self.head.take();
                }
            }
            old_tail.borrow().item
        })
    }

    pub fn push_back(&mut self, item: i32) {
        let new_tail = Node::new(item);
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(old_tail);
                self.tail = Some(new_tail);
            },
            None => {
                self.head = Some(new_tail.clone());
                self.tail = Some(new_tail);
            }
        }
    }
}

impl Default for Deque {
    fn default() -> Self {
        Self { head: None, tail: None }
    }
}

impl IntoIterator for Deque {
    type Item = i32;
    type IntoIter = DequeIter;

    fn into_iter(self) -> Self::IntoIter {
        DequeIter { deque: self }
    }
}

pub struct DequeIter {
    deque: Deque
}

impl Iterator for DequeIter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.deque.pop_front()
    }
}

impl DoubleEndedIterator for DequeIter {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.deque.pop_back()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pop_front_empty() {
        let mut deque = Deque::default();

        assert_eq!(deque.pop_front(), None);
    }

    #[test]
    fn pop_push_front_single() {
        let mut deque = Deque::default();

        deque.push_front(1);

        assert_eq!(deque.pop_front(), Some(1));
        assert_eq!(deque.pop_front(), None);
    }

    #[test]
    fn pop_push_front_many() {
        let mut deque = Deque::default();

        deque.push_front(1);
        deque.push_front(2);
        deque.push_front(3);

        assert_eq!(deque.pop_front(), Some(3));
        assert_eq!(deque.pop_front(), Some(2));
        assert_eq!(deque.pop_front(), Some(1));
        assert_eq!(deque.pop_front(), None);
    }

    #[test]
    fn push_front_pop_back() {
        let mut deque = Deque::default();

        deque.push_front(1);
        deque.push_front(2);
        deque.push_front(3);

        assert_eq!(deque.pop_back(), Some(1));
        assert_eq!(deque.pop_back(), Some(2));
        assert_eq!(deque.pop_back(), Some(3));
        assert_eq!(deque.pop_back(), None);
    }

    #[test]
    fn push_back_pop_front() {
        let mut deque = Deque::default();

        deque.push_back(1);
        deque.push_back(2);
        deque.push_back(3);

        assert_eq!(deque.pop_front(), Some(1));
        assert_eq!(deque.pop_front(), Some(2));
        assert_eq!(deque.pop_front(), Some(3));
        assert_eq!(deque.pop_front(), None);
    }

    #[test]
    fn push_back_pop_back() {
        let mut deque = Deque::default();

        deque.push_back(1);
        deque.push_back(2);
        deque.push_back(3);

        assert_eq!(deque.pop_back(), Some(3));
        assert_eq!(deque.pop_back(), Some(2));
        assert_eq!(deque.pop_back(), Some(1));
        assert_eq!(deque.pop_back(), None);
    }

    #[test]
    fn iterator() {
        let mut deque = Deque::default();

        deque.push_front(1);
        deque.push_front(2);
        deque.push_front(3);

        let mut iter = deque.into_iter();

        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }
    
    #[test]
    fn double_ended_iterator() {
        let mut deque = Deque::default();

        deque.push_front(1);
        deque.push_front(2);
        deque.push_front(3);

        let mut iter = deque.into_iter();

        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next_back(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next_back(), None);
    }
}
