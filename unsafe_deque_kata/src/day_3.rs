use std::ptr::Shared;
use std::marker::PhantomData;

struct Node<T> {
    item: T,
    next: Link<T>,
    prev: Link<T>
}

impl<T> Node<T> {
    fn new(item: T) -> Box<Self> {
        Box::new(Node {
            item: item,
            next: None,
            prev: None
        })
    }
}

type Link<T> = Option<Shared<Node<T>>>;

pub struct UnsafeDeque<T> {
    head: Link<T>,
    tail: Link<T>
}

impl<T> UnsafeDeque<T> {
    pub fn new() -> Self {
        UnsafeDeque { head: None, tail: None }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| unsafe {
            let old_head = Box::from_raw(*old_head);
            self.head = old_head.next;
            match self.head {
                Some(new_head) => { (**new_head).prev.take(); }
                None => { self.tail.take(); }
            }
            old_head.item
        })
    }

    pub fn push_front(&mut self, item: T) {
        let new_head = Node::new(item);
        unsafe {
            let new_head = Shared::new(Box::into_raw(new_head));
            (**new_head).next = self.head;
            match self.head {
                Some(old_head) => { (**old_head).prev = Some(new_head); }
                None => { self.tail = Some(new_head); }
            }
            self.head = Some(new_head);
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| unsafe {
            let old_tail = Box::from_raw(*old_tail);
            self.tail = old_tail.prev;
            match self.tail {
                Some(new_tail) => { (**new_tail).next.take(); }
                None => { self.head.take(); }
            }
            old_tail.item
        })
    }

    pub fn push_back(&mut self, item: T) {
        let new_tail = Node::new(item);
        unsafe {
            let new_tail = Shared::new(Box::into_raw(new_tail));
            (**new_tail).prev = self.tail;
            match self.tail {
                Some(old_tail) => { (**old_tail).next = Some(new_tail); }
                None => { self.head = Some(new_tail); }
            }
            self.tail = Some(new_tail);
        }
    }

    pub fn peek_front(&self) -> Option<&T> {
        self.head.map(|head| unsafe { &(**head).item })
    }

    pub fn peek_mut_front(&mut self) -> Option<&mut T> {
        self.head.map(|head| unsafe { &mut (**head).item })
    }

    pub fn peek_back(&self) -> Option<&T> {
        self.tail.map(|tail| unsafe { &(**tail).item })
    }

    pub fn peek_mut_back(&mut self) -> Option<&mut T> {
        self.tail.map(|tail| unsafe { &mut (**tail).item })
    }
}

impl<T> AsMut<UnsafeDeque<T>> for UnsafeDeque<T> {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl<'mi, T> IntoIterator for &'mi mut UnsafeDeque<T> {
    type Item = &'mi mut T;
    type IntoIter = RefMutDequeIter<'mi, T>;

    fn into_iter(self) -> Self::IntoIter {
        RefMutDequeIter { head: self.head, tail: self.tail, empty: false, marker: PhantomData }
    }
}

pub struct RefMutDequeIter<'mi, T: 'mi> {
    head: Link<T>,
    tail: Link<T>,
    empty: bool,
    marker: PhantomData<&'mi mut T>
}

impl<'mi, T> Iterator for RefMutDequeIter<'mi, T> {
    type Item = &'mi mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.head.and_then(|head| unsafe {
            if self.empty {
                None
            } else {
                self.tail.map(|tail| {
                    self.empty = *head as *const i32 == *tail as *const i32;
                    self.head  = (**head).next;
                    &mut (**head).item
                })
            }
        })
    }
}

impl<'mi, T> DoubleEndedIterator for RefMutDequeIter<'mi, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.tail.and_then(|tail| unsafe{
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

impl<T> AsRef<UnsafeDeque<T>> for UnsafeDeque<T> {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<'i, T> IntoIterator for &'i UnsafeDeque<T> {
    type Item = &'i T;
    type IntoIter = RefDequeIter<'i, T>;

    fn into_iter(self) -> Self::IntoIter {
        RefDequeIter { head: self.head, tail: self.tail, empty: false, marker: PhantomData }
    }
}

pub struct RefDequeIter<'i, T: 'i> {
    head: Link<T>,
    tail: Link<T>,
    empty: bool,
    marker: PhantomData<&'i T>
}

impl<'i, T> Iterator for RefDequeIter<'i, T> {
    type Item = &'i T;

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

impl<'i, T> DoubleEndedIterator for RefDequeIter<'i, T> {
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

impl<T> IntoIterator for UnsafeDeque<T> {
    type Item = T;
    type IntoIter = DequeIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        DequeIter { deque: self }
    }
}

pub struct DequeIter<T> {
    deque: UnsafeDeque<T>
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

#[cfg(test)]
mod tests {
    use super::*;

    fn push_many_in_front(deque: &mut UnsafeDeque<i32>, times: i32) {
        for item in 1..times + 1 {
            deque.push_front(item);
        }
    }

    #[test]
    fn create_an_empty_deque() {
        let mut deque: UnsafeDeque<i32> = UnsafeDeque::new();

        assert_eq!(deque.pop_front(), None);
    }

    #[test]
    fn pushes_one_element_in_front_of_deque() {
        let mut deque = UnsafeDeque::new();

        push_many_in_front(&mut deque, 1);

        assert_eq!(deque.pop_front(), Some(1));
        assert_eq!(deque.pop_front(), None);
    }

    #[test]
    fn pushes_three_elements_in_front_of_deque() {
        let mut deque = UnsafeDeque::new();

        push_many_in_front(&mut deque, 3);

        assert_eq!(deque.pop_front(), Some(3));
        assert_eq!(deque.pop_front(), Some(2));
        assert_eq!(deque.pop_front(), Some(1));
        assert_eq!(deque.pop_front(), None);
    }

    #[test]
    fn pops_from_back_of_deque() {
        let mut deque = UnsafeDeque::new();

        push_many_in_front(&mut deque, 3);

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

        push_many_in_front(&mut deque, 3);

        assert_eq!(deque.peek_front(), Some(&3));
        assert_eq!(deque.peek_front(), Some(&3));
    }

    #[test]
    fn peeks_mut_front() {
        let mut deque = UnsafeDeque::new();

        push_many_in_front(&mut deque, 3);

        assert_eq!(deque.peek_mut_front(), Some(&mut 3));
        assert_eq!(deque.peek_mut_front(), Some(&mut 3));
    }

    #[test]
    fn peeks_back() {
        let mut deque = UnsafeDeque::new();

        push_many_in_front(&mut deque, 3);

        assert_eq!(deque.peek_back(), Some(&1));
        assert_eq!(deque.peek_back(), Some(&1));
    }

    #[test]
    fn peeks_mut_back() {
        let mut deque = UnsafeDeque::new();

        push_many_in_front(&mut deque, 3);

        assert_eq!(deque.peek_mut_back(), Some(&mut 1));
        assert_eq!(deque.peek_mut_back(), Some(&mut 1));
    }

    #[test]
    fn iterate_over_deque() {
        let mut deque = UnsafeDeque::new();

        push_many_in_front(&mut deque, 3);

        let mut iterator = deque.into_iter();

        assert_eq!(iterator.next(), Some(3));
        assert_eq!(iterator.next(), Some(2));
        assert_eq!(iterator.next(), Some(1));
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn double_ended_iterator_over_deque() {
        let mut deque = UnsafeDeque::new();

        push_many_in_front(&mut deque, 3);

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

        push_many_in_front(&mut deque, 3);

        let mut iterator = deque.as_ref().into_iter();

        assert_eq!(iterator.next(), Some(&3));
        assert_eq!(iterator.next(), Some(&2));
        assert_eq!(iterator.next(), Some(&1));
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn double_ended_ref_iterator_over_deque() {
        let mut deque = UnsafeDeque::new();

        push_many_in_front(&mut deque, 3);

        let mut iterator = deque.as_ref().into_iter();

        assert_eq!(iterator.next(), Some(&3));
        assert_eq!(iterator.next_back(), Some(&1));
        assert_eq!(iterator.next_back(), Some(&2));
        assert_eq!(iterator.next_back(), None);
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn rev_mut_iterator_over_deque() {
        let mut deque = UnsafeDeque::new();

        push_many_in_front(&mut deque, 3);

        let mut iterator = deque.as_mut().into_iter();

        assert_eq!(iterator.next(), Some(&mut 3));
        assert_eq!(iterator.next(), Some(&mut 2));
        assert_eq!(iterator.next(), Some(&mut 1));
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn double_ended_ref_mut_iterator_over_deque() {
        let mut deque = UnsafeDeque::new();

        push_many_in_front(&mut deque, 3);

        let mut iterator = deque.as_mut().into_iter();

        assert_eq!(iterator.next(), Some(&mut 3));
        assert_eq!(iterator.next_back(), Some(&mut 1));
        assert_eq!(iterator.next_back(), Some(&mut 2));
        assert_eq!(iterator.next_back(), None);
        assert_eq!(iterator.next(), None);
    }
}
