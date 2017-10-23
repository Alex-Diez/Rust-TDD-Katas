use std::ptr::Shared;
use std::iter::FromIterator;
use std::marker::PhantomData;

type Link = Option<Shared<Node>>;

struct Node {
    item: i32,
    next: Link,
    prev: Link
}

impl Node {
    fn new(item: i32) -> Box<Self> {
        Box::new(Node {
            item: item,
            next: None,
            prev: None
        })
    }
}

pub struct UnsafeDeque {
    head: Link,
    tail: Link
}

impl UnsafeDeque {
    pub fn new() -> Self {
        UnsafeDeque { head: None, tail: None }
    }

    pub fn pop_front(&mut self) -> Option<i32> {
        self.head.take().map(|old_head| unsafe {
            let mut old_head = Box::from_raw(*old_head);
            self.head = (*old_head).next.take();
            match self.head {
                Some(new_head) => { (**new_head).prev.take(); },
                None => { self.tail.take(); }
            }
            (*old_head).item
        })
    }

    pub fn push_front(&mut self, item: i32) {
        unsafe {
            let new_head = Shared::new(Box::into_raw(Node::new(item)));
            (**new_head).next = self.head;
            match self.head {
                Some(old_head) => (**old_head).prev = Some(new_head),
                None => self.tail = Some(new_head)
            }
            self.head = Some(new_head);
        }
    }

    pub fn pop_back(&mut self) -> Option<i32> {
        self.tail.take().map(|old_tail| unsafe {
            let mut old_tail = Box::from_raw(*old_tail);
            self.tail = (*old_tail).prev.take();
            match self.tail {
                Some(new_tail) => { (**new_tail).next.take(); }
                None => { self.head.take(); }
            }
            (*old_tail).item
        })
    }

    pub fn push_back(&mut self, item: i32) {
        unsafe {
            let new_tail = Shared::new(Box::into_raw(Node::new(item)));
            (**new_tail).prev = self.tail;
            match self.tail {
                Some(old_tail) => (**old_tail).next = Some(new_tail),
                None => self.head = Some(new_tail)
            }
            self.tail = Some(new_tail);
        }
    }

    pub fn peek_front(&self) -> Option<&i32> {
        self.head.map(|head| unsafe { &(**head).item })
    }

    pub fn peek_mut_front(&mut self) -> Option<&mut i32> {
        self.head.map(|head| unsafe { &mut (**head).item })
    }

    pub fn peek_back(&self) -> Option<&i32> {
        self.tail.map(|tail| unsafe { &(**tail).item })
    }

    pub fn peek_mut_back(&mut self) -> Option<&mut i32> {
        self.tail.map(|tail| unsafe { &mut (**tail).item })
    }
}

impl AsMut<UnsafeDeque> for UnsafeDeque {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl<'mi> IntoIterator for &'mi mut UnsafeDeque {
    type Item = &'mi mut i32;
    type IntoIter = RefMutDequeIter<'mi>;

    fn into_iter(self) -> Self::IntoIter {
        RefMutDequeIter { head: self.head, tail: self.tail, empty: false, marker: PhantomData }
    }
}

pub struct RefMutDequeIter<'mi> {
    head: Link,
    tail: Link,
    empty: bool,
    marker: PhantomData<&'mi mut i32>
}

impl<'mi> Iterator for RefMutDequeIter<'mi> {
    type Item = &'mi mut i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.head.take().and_then(|head| unsafe {
            if self.empty {
                None
            } else {
                self.tail.map(|tail| {
                    self.empty = *head as *const i32 == *tail as *const i32;
                    self.head = (**head).next;
                    &mut (**head).item
                })
            }
        })
    }
}

impl<'mi> DoubleEndedIterator for RefMutDequeIter<'mi> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.tail.take().and_then(|tail| unsafe {
            if self.empty {
                None
            } else {
                self.head.map(|head| {
                    self.empty = *head as *const i32 == *tail as *const i32;
                    self.tail = (**tail).prev;
                    &mut (**tail).item
                })
            }
        })
    }
}

impl AsRef<UnsafeDeque> for UnsafeDeque {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<'i> IntoIterator for &'i UnsafeDeque {
    type Item = &'i i32;
    type IntoIter = RefDequeIter<'i>;

    fn into_iter(self) -> Self::IntoIter {
        RefDequeIter { head: self.head, tail: self.tail, empty: false, marker: PhantomData }
    }
}

pub struct RefDequeIter<'i> {
    head: Link,
    tail: Link,
    empty: bool,
    marker: PhantomData<&'i i32>
}

impl<'i> Iterator for RefDequeIter<'i> {
    type Item = &'i i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.head.take().and_then(|head| unsafe {
            if self.empty {
                None
            } else {
                self.tail.map(|tail| {
                    self.empty = *head as *const i32 == *tail as *const i32;
                    self.head = (**head).next;
                    &(**head).item
                })
            }
        })
    }
}

impl<'i> DoubleEndedIterator for RefDequeIter<'i> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.tail.take().and_then(|tail| unsafe {
            if self.empty {
                None
            } else {
                self.head.map(|head| {
                    self.empty = *head as *const i32 == *tail as *const i32;
                    self.tail = (**tail).prev;
                    &(**tail).item
                })
            }
        })
    }
}

impl IntoIterator for UnsafeDeque {
    type Item = i32;
    type IntoIter = DequeIter;

    fn into_iter(self) -> Self::IntoIter {
        DequeIter { deque: self }
    }
}

pub struct DequeIter {
    deque: UnsafeDeque
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

impl FromIterator<i32> for UnsafeDeque {
    fn from_iter<I: IntoIterator<Item = i32>>(items: I) -> Self {
        let mut deque = UnsafeDeque::new();
        for item in items {
            deque.push_front(item);
        }
        deque
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_an_empty_deque() {
        let mut deque = UnsafeDeque::new();

        assert_eq!(deque.pop_front(), None);
    }

    #[test]
    fn pushes_one_element_in_front() {
        let mut deque = UnsafeDeque::new();

        deque.push_front(1);

        assert_eq!(deque.pop_front(), Some(1));
        assert_eq!(deque.pop_front(), None);
    }

    #[test]
    fn pushes_three_elements_in_front() {
        let mut deque = UnsafeDeque::new();

        deque.push_front(1);
        deque.push_front(2);
        deque.push_front(3);

        assert_eq!(deque.pop_front(), Some(3));
        assert_eq!(deque.pop_front(), Some(2));
        assert_eq!(deque.pop_front(), Some(1));
        assert_eq!(deque.pop_front(), None);
    }

    #[test]
    fn pops_from_back() {
        let mut deque = UnsafeDeque::from_iter(1..4);

        assert_eq!(deque.pop_back(), Some(1));
        assert_eq!(deque.pop_back(), Some(2));
        assert_eq!(deque.pop_back(), Some(3));
        assert_eq!(deque.pop_back(), None);
    }

    #[test]
    fn pushes_three_elements_in_back() {
        let mut deque = UnsafeDeque::new();

        deque.push_back(1);
        deque.push_back(2);
        deque.push_back(3);

        assert_eq!(deque.pop_front(), Some(1));
        assert_eq!(deque.pop_front(), Some(2));
        assert_eq!(deque.pop_front(), Some(3));
        assert_eq!(deque.pop_front(), None);
    }

    #[test]
    fn peeks_front() {
        let deque = UnsafeDeque::from_iter(1..4);

        assert_eq!(deque.peek_front(), Some(&3));
        assert_eq!(deque.peek_front(), Some(&3));
    }

    #[test]
    fn peeks_mut_front() {
        let mut deque = UnsafeDeque::from_iter(1..4);

        assert_eq!(deque.peek_mut_front(), Some(&mut 3));
        assert_eq!(deque.peek_mut_front(), Some(&mut 3));
    }

    #[test]
    fn peeks_back() {
        let deque = UnsafeDeque::from_iter(1..4);

        assert_eq!(deque.peek_back(), Some(&1));
        assert_eq!(deque.peek_back(), Some(&1));
    }

    #[test]
    fn peeks_mut_back() {
        let mut deque = UnsafeDeque::from_iter(1..4);

        assert_eq!(deque.peek_mut_back(), Some(&mut 1));
        assert_eq!(deque.peek_mut_back(), Some(&mut 1));
    }

    #[test]
    fn iterator_over_deque() {
        let deque = UnsafeDeque::from_iter(1..4);

        let mut iterator = deque.into_iter();

        assert_eq!(iterator.next(), Some(3));
        assert_eq!(iterator.next(), Some(2));
        assert_eq!(iterator.next(), Some(1));
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn double_ended_iterator_over_deque() {
        let deque = UnsafeDeque::from_iter(1..4);

        let mut iterator = deque.into_iter();

        assert_eq!(iterator.next(), Some(3));
        assert_eq!(iterator.next_back(), Some(1));
        assert_eq!(iterator.next_back(), Some(2));
        assert_eq!(iterator.next_back(), None);
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn ref_iterator_over_deque() {
        let deque = UnsafeDeque::from_iter(1..4);

        let mut iterator = deque.as_ref().into_iter();

        assert_eq!(iterator.next(), Some(&3));
        assert_eq!(iterator.next(), Some(&2));
        assert_eq!(iterator.next(), Some(&1));
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn double_ended_ref_iterator_over_deque() {
        let deque = UnsafeDeque::from_iter(1..4);

        let mut iterator = deque.as_ref().into_iter();

        assert_eq!(iterator.next(), Some(&3));
        assert_eq!(iterator.next_back(), Some(&1));
        assert_eq!(iterator.next_back(), Some(&2));
        assert_eq!(iterator.next_back(), None);
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn mut_ref_iterator_over_deque() {
        let mut deque = UnsafeDeque::from_iter(1..4);

        let mut iterator = deque.as_mut().into_iter();

        assert_eq!(iterator.next(), Some(&mut 3));
        assert_eq!(iterator.next(), Some(&mut 2));
        assert_eq!(iterator.next(), Some(&mut 1));
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn double_ended_ref_mut_iterator_over_deque() {
        let mut deque = UnsafeDeque::from_iter(1..4);

        let mut iterator = deque.as_mut().into_iter();

        assert_eq!(iterator.next(), Some(&mut 3));
        assert_eq!(iterator.next_back(), Some(&mut 1));
        assert_eq!(iterator.next_back(), Some(&mut 2));
        assert_eq!(iterator.next_back(), None);
        assert_eq!(iterator.next(), None);
    }
}
