use std::cell::RefCell;
use std::rc::Rc;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    item: T,
    next: Link<T>,
    prev: Link<T>
}

impl<T> Node<T> {
    fn new(item: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node { item, next: None, prev: None }))
    }
}

pub struct Deque<T> {
    head: Link<T>,
    tail: Link<T>
}

impl<T> Deque<T> {
    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().and_then(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev.take();
                    self.head = Some(new_head);
                }
                None => {
                    self.tail.take();
                }
            }
            Rc::try_unwrap(old_head).ok().map(|r| r.into_inner().item)
        })
    }

    pub fn push_front(&mut self, item: T) {
        let new_head = Node::new(item);
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head);
            },
            None => {
                self.tail = Some(new_head.clone());
            }
        }
        self.head = Some(new_head)
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().and_then(|old_tail| {
            match old_tail.borrow_mut().prev.take() {
                Some(new_tail) => {
                    new_tail.borrow_mut().next.take();
                    self.tail = Some(new_tail);
                }
                None => {
                    self.head.take();
                }
            }
            Rc::try_unwrap(old_tail).ok().map(|r| r.into_inner().item)
        })
    }

    pub fn push_back(&mut self, item: T) {
        let new_tail = Node::new(item);
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(old_tail);
            }
            None => {
                self.head = Some(new_tail.clone());
            }
        }
        self.tail = Some(new_tail);
    }
}

impl<T> Default for Deque<T> {
    fn default() -> Self {
        Self { head: None, tail: None }
    }
}

impl<T> IntoIterator for Deque<T> {
    type Item = T;
    type IntoIter = DequeIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        DequeIter { deque: self }
    }
}

pub struct DequeIter<T> {
    deque: Deque<T>
}

impl<T> Iterator for DequeIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.deque.pop_front()
    }
}

impl<T> DoubleEndedIterator for DequeIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.deque.pop_back()
    }
}

impl<T> Drop for Deque<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop_front() {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pop_front_from_empty_deque() {
        let mut deque: Deque<i32> = Deque::default();

        assert_eq!(deque.pop_front(), None);
    }

    #[test]
    fn push_pop_front_single_item() {
        let mut deque = Deque::default();

        deque.push_front(1);

        assert_eq!(deque.pop_front(), Some(1));
        assert_eq!(deque.pop_front(), None);
    }

    #[test]
    fn push_pop_front_many_items() {
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
