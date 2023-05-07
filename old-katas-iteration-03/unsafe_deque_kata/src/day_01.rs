use std::ptr::NonNull;

type Link<T> = Option<NonNull<Node<T>>>;

struct Node<T> {
    item: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> Node<T> {
    fn new(item: T) -> Box<Self> {
        Box::new(Node{ item, next: None, prev: None })
    }
}

pub struct UnsafeDeque<T: Clone> {
    head: Link<T>,
    tail: Link<T>,
}

impl<T: Clone> UnsafeDeque<T> {
    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|mut old_head| unsafe {
            match old_head.as_mut().next.take() {
                Some(mut new_head) => {
                    new_head.as_mut().prev.take();
                    self.head = Some(new_head);
                }
                None => {
                    self.tail.take();
                }
            }
            old_head.as_mut().item.clone()
        })
    }

    pub fn push_front(&mut self, item: T) {
        if let Some(mut new_head) = NonNull::new(Box::into_raw(Node::new(item))) {
            unsafe {
                new_head.as_mut().next = self.head;
                match self.head.take() {
                    Some(mut old_head) => old_head.as_mut().prev = Some(new_head),
                    None => self.tail = Some(new_head)
                }
                self.head = Some(new_head);
            }
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|mut old_tail| unsafe {
            match old_tail.as_mut().prev.take() {
                Some(mut new_tail) => {
                    new_tail.as_mut().next.take();
                    self.tail = Some(new_tail);
                }
                None => {
                    self.head.take();
                }
            }
            old_tail.as_mut().item.clone()
        })
    }

    pub fn push_back(&mut self, item: T) {
        if let Some(mut new_tail) = NonNull::new(Box::into_raw(Node::new(item))) {
            unsafe {
                new_tail.as_mut().prev = self.tail;
                match self.tail.take() {
                    Some(mut old_tail) => old_tail.as_mut().next = Some(new_tail),
                    None => self.head = Some(new_tail)
                }
                self.tail = Some(new_tail);
            }
        }
    }
}

impl<T: Clone> Default for UnsafeDeque<T> {
    fn default() -> Self {
        Self { head: None, tail: None }
    }
}

impl<T: Clone> IntoIterator for UnsafeDeque<T> {
    type Item = T;
    type IntoIter = UnsafeDequeIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        UnsafeDequeIter { deque: self }
    }
}

pub struct UnsafeDequeIter<T: Clone> {
    deque: UnsafeDeque<T>
}

impl<T: Clone> Iterator for UnsafeDequeIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.deque.pop_front()
    }
}

impl<T: Clone> DoubleEndedIterator for UnsafeDequeIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.deque.pop_back()
    }
}

impl<T: Clone> Drop for UnsafeDeque<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop_front() {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pop_front_from_empty_deque() {
        let mut deque: UnsafeDeque<i32> = UnsafeDeque::default();

        assert_eq!(deque.pop_front(), None);
    }

    #[test]
    fn push_front_pop_front_single_item() {
        let mut deque = UnsafeDeque::default();

        deque.push_front(1);

        assert_eq!(deque.pop_front(), Some(1));
        assert_eq!(deque.pop_front(), None);
    }

    #[test]
    fn push_front_pop_front_many_items() {
        let mut deque = UnsafeDeque::default();

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
        let mut deque = UnsafeDeque::default();

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
        let mut deque = UnsafeDeque::default();

        deque.push_back(1);
        deque.push_back(2);
        deque.push_back(3);

        assert_eq!(deque.pop_front(), Some(1));
        assert_eq!(deque.pop_front(), Some(2));
        assert_eq!(deque.pop_front(), Some(3));
        assert_eq!(deque.pop_front(), None);
    }

    #[test]
    fn iterator() {
        let mut deque = UnsafeDeque::default();

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
        let mut deque = UnsafeDeque::default();

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
