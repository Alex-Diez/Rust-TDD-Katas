use std::ptr::Shared;
use std::marker::PhantomData;

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

type Link = Option<Shared<Node>>;

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
            let old_head = Box::from_raw(*old_head);
            self.head = (*old_head).next;
            match self.head {
                Some(new_head) => { (**new_head).prev.take(); }
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
                Some(old_head) => { (**old_head).prev = Some(new_head); }
                None => { self.tail = Some(new_head); }
            }
            self.head = Some(new_head);
        }
    }

    pub fn pop_back(&mut self) -> Option<i32> {
        self.tail.take().map(|old_tail| unsafe {
            let old_tail = Box::from_raw(*old_tail);
            self.tail = (*old_tail).prev;
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
                Some(old_tail) => { (**old_tail).next = Some(new_tail); }
                None => { self.head = Some(new_tail); }
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

    pub fn as_mut(&mut self) -> &mut Self {
        self
    }

    pub fn into_iter(&mut self) -> RefMutDequeIter {
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
        self.head.and_then(|head| unsafe {
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
        self.tail.and_then(|tail| unsafe {
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
        self.head.and_then(|head| unsafe {
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
        self.tail.and_then(|tail| unsafe {
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

    fn  into_iter(self) -> DequeIter {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn pushes_many_in_front(deque: &mut UnsafeDeque, times: i32) {
        for item in 1..times + 1 {
            deque.push_front(item);
        }
    }

    #[test]
    fn creates_an_empty_deque() {
        let mut deque = UnsafeDeque::new();

        assert_eq!(deque.pop_front(), None);
    }

    #[test]
    fn pushes_one_element_in_front_of_deque() {
        let mut deque = UnsafeDeque::new();

        pushes_many_in_front(&mut deque, 1);

        assert_eq!(deque.pop_front(), Some(1));
        assert_eq!(deque.pop_front(), None);
    }

    #[test]
    fn pushes_three_elements_in_front_of_deque() {
        let mut deque = UnsafeDeque::new();

        pushes_many_in_front(&mut deque, 3);

        assert_eq!(deque.pop_front(), Some(3));
        assert_eq!(deque.pop_front(), Some(2));
        assert_eq!(deque.pop_front(), Some(1));
        assert_eq!(deque.pop_front(), None);
    }

    #[test]
    fn pops_from_back_of_deque() {
        let mut deque = UnsafeDeque::new();

        pushes_many_in_front(&mut deque, 3);

        assert_eq!(deque.pop_back(), Some(1));
        assert_eq!(deque.pop_back(), Some(2));
        assert_eq!(deque.pop_back(), Some(3));
        assert_eq!(deque.pop_back(), None);
    }

    #[test]
    fn pushes_three_elements_in_back_of_deque() {
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
        let mut deque = UnsafeDeque::new();

        pushes_many_in_front(&mut deque, 3);

        assert_eq!(deque.peek_front(), Some(&3));
        assert_eq!(deque.peek_front(), Some(&3));
    }

    #[test]
    fn peeks_mut_front() {
        let mut deque = UnsafeDeque::new();

        pushes_many_in_front(&mut deque, 3);

        assert_eq!(deque.peek_mut_front(), Some(&mut 3));
        assert_eq!(deque.peek_mut_front(), Some(&mut 3));
    }

    #[test]
    fn peeks_back() {
        let mut deque = UnsafeDeque::new();

        pushes_many_in_front(&mut deque, 3);

        assert_eq!(deque.peek_back(), Some(&1));
        assert_eq!(deque.peek_back(), Some(&1));
    }

    #[test]
    fn peeks_mut_back() {
        let mut deque = UnsafeDeque::new();

        pushes_many_in_front(&mut deque, 3);

        assert_eq!(deque.peek_mut_back(), Some(&mut 1));
        assert_eq!(deque.peek_mut_back(), Some(&mut 1));
    }

    #[test]
    fn iterator_over_deque() {
        let mut deque = UnsafeDeque::new();

        pushes_many_in_front(&mut deque, 3);

        let mut iterator = deque.into_iter();

        assert_eq!(iterator.next(), Some(3));
        assert_eq!(iterator.next(), Some(2));
        assert_eq!(iterator.next(), Some(1));
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn double_ended_iterator_over_deque() {
        let mut deque = UnsafeDeque::new();

        pushes_many_in_front(&mut deque, 3);

        let mut iterator = deque.into_iter();

        assert_eq!(iterator.next(), Some(3));
        assert_eq!(iterator.next_back(), Some(1));
        assert_eq!(iterator.next_back(), Some(2));
        assert_eq!(iterator.next_back(), None);
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn ref_iterator_over_deque() {
        let mut deque = UnsafeDeque::new();

        pushes_many_in_front(&mut deque, 3);

        let mut iterator = deque.as_ref().into_iter();

        assert_eq!(iterator.next(), Some(&3));
        assert_eq!(iterator.next(), Some(&2));
        assert_eq!(iterator.next(), Some(&1));
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn double_ended_ref_iterator_over_deque() {
        let mut deque = UnsafeDeque::new();

        pushes_many_in_front(&mut deque, 3);

        let mut iterator = deque.as_ref().into_iter();

        assert_eq!(iterator.next(), Some(&3));
        assert_eq!(iterator.next_back(), Some(&1));
        assert_eq!(iterator.next_back(), Some(&2));
        assert_eq!(iterator.next_back(), None);
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn ref_mut_iterator_over_deque() {
        let mut deque = UnsafeDeque::new();

        pushes_many_in_front(&mut deque, 3);

        let mut iterator = deque.as_mut().into_iter();

        assert_eq!(iterator.next(), Some(&mut 3));
        assert_eq!(iterator.next(), Some(&mut 2));
        assert_eq!(iterator.next(), Some(&mut 1));
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn double_mut_ref_iterator_over_deque() {
        let mut deque = UnsafeDeque::new();

        pushes_many_in_front(&mut deque, 3);

        let mut iterator = deque.as_mut().into_iter();

        assert_eq!(iterator.next(), Some(&mut 3));
        assert_eq!(iterator.next_back(), Some(&mut 1));
        assert_eq!(iterator.next_back(), Some(&mut 2));
        assert_eq!(iterator.next_back(), None);
        assert_eq!(iterator.next(), None);
    }
}
