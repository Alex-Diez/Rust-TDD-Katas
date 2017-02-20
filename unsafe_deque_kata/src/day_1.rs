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
            let node = Box::from_raw(*old_head);
            self.head = node.next;
            match self.head {
                Some(new_head) => { (**new_head).prev.take(); }
                None => { self.tail.take(); }
            }
            node.item
        })
    }

    pub fn push_front(&mut self, item: T) {
        let mut new_head = Node::new(item);
        unsafe {
            new_head.next = self.head;
            let new_head = Some(Shared::new(Box::into_raw(new_head)));
            match self.head {
                Some(old_head) => (**old_head).prev = new_head,
                None => self.tail = new_head
            }
            self.head = new_head;
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| unsafe {
            let node = Box::from_raw(*old_tail);
            self.tail = node.prev;
            match self.tail {
                Some(new_tail) => { (**new_tail).next.take(); }
                None => { self.head.take(); }
            }
            node.item
        })
    }

    pub fn push_back(&mut self, item: T) {
        let mut new_tail = Node::new(item);
        unsafe {
            new_tail.prev = self.tail;
            let new_tail = Some(Shared::new(Box::into_raw(new_tail)));
            match self.tail {
                Some(old_tail) => (**old_tail).next = new_tail,
                None => self.head = new_tail,
            }
            self.tail = new_tail;
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
    type IntoIter = RefMutDequeIterator<'mi, T>;

    fn into_iter(self) -> Self::IntoIter {
        RefMutDequeIterator { head: self.head, tail: self.tail, marker: PhantomData, empty: false }
    }
}

pub struct RefMutDequeIterator<'mi, T: 'mi> {
    head: Link<T>,
    tail: Link<T>,
    marker: PhantomData<&'mi mut T>,
    empty: bool
}

impl<'mi, T> Iterator for RefMutDequeIterator<'mi, T> {
    type Item = &'mi mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.head.and_then(|head| unsafe {
            if self.empty {
                None
            } else {
                self.tail.map(|tail| {
                    self.empty = *tail as *const i32 == *head as *const i32;
                    self.head = (**head).next;
                    &mut (**head).item
                })
            }
        })
    }
}

impl<'mi, T> DoubleEndedIterator for RefMutDequeIterator<'mi, T> {
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

impl<T> AsRef<UnsafeDeque<T>> for UnsafeDeque<T> {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<'i, T> IntoIterator for &'i UnsafeDeque<T> {
    type Item = &'i T;
    type IntoIter = RefDequeIterator<'i, T>;

    fn into_iter(self) -> Self::IntoIter {
        RefDequeIterator { head: self.head, tail: self.tail, marker: PhantomData, empty: false }
    }
}

pub struct RefDequeIterator<'i, T: 'i> {
    head: Link<T>,
    tail: Link<T>,
    marker: PhantomData<&'i T>,
    empty: bool
}

impl<'i, T> Iterator for RefDequeIterator<'i, T> {
    type Item = &'i T;

    fn next(&mut self) -> Option<Self::Item> {
        self.head.and_then(|head| unsafe {
            if self.empty {
                None
            } else {
                self.tail.map(|tail| {
                    self.empty = *tail as *const i32 == *head as *const i32;
                    self.head = (**head).next;
                    &(**head).item
                })
            }
        })
    }
}

impl<'i, T> DoubleEndedIterator for RefDequeIterator<'i, T> {
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
    type IntoIter = DequeIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        DequeIterator { deque: self }
    }
}

pub struct DequeIterator<T> {
    deque: UnsafeDeque<T>
}

impl<T> Iterator for DequeIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.deque.pop_front()
    }
}

impl<T> DoubleEndedIterator for DequeIterator<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.deque.pop_back()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn push_many_front(deque: &mut UnsafeDeque<i32>, times: i32) {
        for item in 1..times + 1 {
            deque.push_front(item);
        }
    }

    #[test]
    fn creates_an_empty_deque() {
        let mut deque: UnsafeDeque<i32> = UnsafeDeque::new();

        assert_eq!(deque.pop_front(), None);
    }

    #[test]
    fn pushes_one_element_in_front_of_deque() {
        let mut deque = UnsafeDeque::new();

        push_many_front(&mut deque, 1);

        assert_eq!(deque.pop_front(), Some(1));
    }

    #[test]
    fn pushes_three_elements_in_front_of_deque() {
        let mut deque = UnsafeDeque::new();

        push_many_front(&mut deque, 3);

        assert_eq!(deque.pop_front(), Some(3));
        assert_eq!(deque.pop_front(), Some(2));
        assert_eq!(deque.pop_front(), Some(1));
        assert_eq!(deque.pop_front(), None);
    }

    #[test]
    fn pops_from_back_of_deque() {
        let mut deque = UnsafeDeque::new();

        push_many_front(&mut deque, 3);

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

        push_many_front(&mut deque, 3);

        assert_eq!(deque.peek_front(), Some(&3));
        assert_eq!(deque.peek_front(), Some(&3));
    }

    #[test]
    fn peeks_mut_front() {
        let mut deque = UnsafeDeque::new();

        push_many_front(&mut deque, 3);

        assert_eq!(deque.peek_mut_front(), Some(&mut 3));
        assert_eq!(deque.peek_mut_front(), Some(&mut 3));
    }

    #[test]
    fn peeks_back() {
        let mut deque = UnsafeDeque::new();

        push_many_front(&mut deque, 3);

        assert_eq!(deque.peek_back(), Some(&1));
        assert_eq!(deque.peek_back(), Some(&1));
    }

    #[test]
    fn peeks_mut_back() {
        let mut deque = UnsafeDeque::new();

        push_many_front(&mut deque, 3);

        assert_eq!(deque.peek_mut_back(), Some(&mut 1));
        assert_eq!(deque.peek_mut_back(), Some(&mut 1));
    }

    #[test]
    fn iterator_over_deque() {
        let mut deque = UnsafeDeque::new();

        push_many_front(&mut deque, 3);

        let mut iterator = deque.into_iter();

        assert_eq!(iterator.next(), Some(3));
        assert_eq!(iterator.next(), Some(2));
        assert_eq!(iterator.next(), Some(1));
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn ref_iterator_over_deque() {
        let mut deque = UnsafeDeque::new();

        push_many_front(&mut deque, 3);

        let mut iterator = deque.as_ref().into_iter();

        assert_eq!(iterator.next(), Some(&3));
        assert_eq!(iterator.next(), Some(&2));
        assert_eq!(iterator.next(), Some(&1));
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn ref_mut_iterator_over_deque() {
        let mut deque = UnsafeDeque::new();

        push_many_front(&mut deque, 3);

        let mut iterator = deque.as_mut().into_iter();

        assert_eq!(iterator.next(), Some(&mut 3));
        assert_eq!(iterator.next(), Some(&mut 2));
        assert_eq!(iterator.next(), Some(&mut 1));
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn double_ended_iterator_over_deque() {
        let mut deque = UnsafeDeque::new();

        push_many_front(&mut deque, 3);

        let mut iterator = deque.into_iter();

        assert_eq!(iterator.next(), Some(3));
        assert_eq!(iterator.next_back(), Some(1));
        assert_eq!(iterator.next_back(), Some(2));
        assert_eq!(iterator.next_back(), None);
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn double_ended_ref_iterator_over_deque() {
        let mut deque = UnsafeDeque::new();

        push_many_front(&mut deque, 3);

        let mut iterator = deque.as_ref().into_iter();

        assert_eq!(iterator.next(), Some(&3));
        assert_eq!(iterator.next_back(), Some(&1));
        assert_eq!(iterator.next_back(), Some(&2));
        assert_eq!(iterator.next_back(), None);
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn double_ended_ref_mut_iterator_over_deque() {
        let mut deque = UnsafeDeque::new();

        push_many_front(&mut deque, 3);

        let mut iterator = deque.as_mut().into_iter();

        assert_eq!(iterator.next(), Some(&mut 3));
        assert_eq!(iterator.next_back(), Some(&mut 1));
        assert_eq!(iterator.next_back(), Some(&mut 2));
        assert_eq!(iterator.next_back(), None);
        assert_eq!(iterator.next(), None);
    }
}
