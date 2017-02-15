use std::rc::Rc;
use std::cell::{RefCell, Ref, RefMut};

struct Node {
    item: i32,
    next: Link,
    prev: Link
}

impl Node {
    fn new(item: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            item: item,
            next: None,
            prev: None
        }))
    }
}

type Link = Option<Rc<RefCell<Node>>>;

pub struct Deque {
    head: Link,
    tail: Link
}

impl Deque {
    pub fn new() -> Self {
        Deque { head: None, tail: None }
    }

    pub fn pop_front(&mut self) -> Option<i32> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev.take();
                    self.head = Some(new_head);
                }
                None => { self.tail.take(); }
            }
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().item
        })
    }

    pub fn push_front(&mut self, item: i32) {
        let new_head = Node::new(item);
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head);
                self.head = Some(new_head);
            }
            None => {
                self.head = Some(new_head.clone());
                self.tail = Some(new_head);
            }
        }
    }

    pub fn pop_back(&mut self) -> Option<i32> {
        self.tail.take().map(|old_tail| {
            match old_tail.borrow_mut().prev.take() {
                Some(new_tail) => {
                    new_tail.borrow_mut().next.take();
                    self.tail = Some(new_tail);
                }
                None => { self.head.take(); }
            }
            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().item
        })
    }

    pub fn push_back(&mut self, item: i32) {
        let new_tail = Node::new(item);
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(old_tail);
                self.tail = Some(new_tail);
            }
            None => {
                self.head = Some(new_tail.clone());
                self.tail = Some(new_tail);
            }
        }
    }

    pub fn peek_front(&self) -> Option<Ref<i32>> {
        self.head.as_ref().map(|node| {
            Ref::map(node.borrow(), |node| &node.item)
        })
    }

    pub fn peek_mut_front(&mut self) -> Option<RefMut<i32>> {
        self.head.as_mut().map(|node| {
            RefMut::map(node.borrow_mut(), |node| &mut node.item)
        })
    }

    pub fn peek_back(&self) -> Option<Ref<i32>> {
        self.tail.as_ref().map(|node| {
            Ref::map(node.borrow(), |node| &node.item)
        })
    }

    pub fn peek_mut_back(&mut self) -> Option<RefMut<i32>> {
        self.tail.as_mut().map(|node| {
            RefMut::map(node.borrow_mut(), |node| &mut node.item)
        })
    }
}

impl IntoIterator for Deque {
    type Item = i32;
    type IntoIter = DequeIterator;

    fn into_iter(self) -> Self::IntoIter {
        DequeIterator { deque: self }
    }
}

pub struct DequeIterator {
    deque: Deque
}

impl Iterator for DequeIterator {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.deque.pop_front()
    }
}

impl DoubleEndedIterator for DequeIterator {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.deque.pop_back()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn push_many_in_front(deque: &mut Deque, times: i32) {
        for item in 1..times + 1 {
            deque.push_front(item)
        }
    }

    #[test]
    fn creates_an_empty_deque() {
        let mut deque = Deque::new();

        assert_eq!(deque.pop_front(), None);
    }

    #[test]
    fn pushes_one_element_in_front_of_deque() {
        let mut deque = Deque::new();

        push_many_in_front(&mut deque, 1);

        assert_eq!(deque.pop_front(), Some(1));
        assert_eq!(deque.pop_front(), None);
    }

    #[test]
    fn pushes_three_elements_in_front_of_deque() {
        let mut deque = Deque::new();

        push_many_in_front(&mut deque, 3);

        assert_eq!(deque.pop_front(), Some(3));
        assert_eq!(deque.pop_front(), Some(2));
        assert_eq!(deque.pop_front(), Some(1));
        assert_eq!(deque.pop_front(), None);
    }

    #[test]
    fn pops_back_from_deque() {
        let mut deque = Deque::new();

        push_many_in_front(&mut deque, 3);

        assert_eq!(deque.pop_back(), Some(1));
        assert_eq!(deque.pop_back(), Some(2));
        assert_eq!(deque.pop_back(), Some(3));
        assert_eq!(deque.pop_back(), None);
    }

    #[test]
    fn pushes_three_elements_in_back_of_deque() {
        let mut deque = Deque::new();

        deque.push_back(1);
        deque.push_back(2);
        deque.push_back(3);

        assert_eq!(deque.pop_front(), Some(1));
        assert_eq!(deque.pop_front(), Some(2));
        assert_eq!(deque.pop_front(), Some(3));
        assert_eq!(deque.pop_front(), None);
    }

    #[test]
    fn peeks_front_from_deque() {
        let mut deque = Deque::new();

        push_many_in_front(&mut deque, 3);

        assert_eq!(&*deque.peek_front().unwrap(), &3);
        assert_eq!(&*deque.peek_front().unwrap(), &3);
    }

    #[test]
    fn peeks_mut_front_from_deque() {
        let mut deque = Deque::new();

        push_many_in_front(&mut deque, 3);

        assert_eq!(&*deque.peek_mut_front().unwrap(), &mut 3);
        assert_eq!(&*deque.peek_mut_front().unwrap(), &mut 3);
    }

    #[test]
    fn peeks_back_from_deque() {
        let mut deque = Deque::new();

        push_many_in_front(&mut deque, 3);

        assert_eq!(&*deque.peek_back().unwrap(), &1);
        assert_eq!(&*deque.peek_back().unwrap(), &1);
    }

    #[test]
    fn peeks_mut_back_from_deque() {
        let mut deque = Deque::new();

        push_many_in_front(&mut deque, 3);

        assert_eq!(&*deque.peek_mut_back().unwrap(), &mut 1);
        assert_eq!(&*deque.peek_mut_back().unwrap(), &mut 1);
    }

    #[test]
    fn iterates_over_deque() {
        let mut deque = Deque::new();

        push_many_in_front(&mut deque, 3);

        let mut iterator = deque.into_iter();

        assert_eq!(iterator.next(), Some(3));
        assert_eq!(iterator.next(), Some(2));
        assert_eq!(iterator.next(), Some(1));
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn double_ended_iterator_over_deque() {
        let mut deque = Deque::new();

        push_many_in_front(&mut deque, 3);

        let mut iterator = deque.into_iter();

        assert_eq!(iterator.next_back(), Some(1));
        assert_eq!(iterator.next_back(), Some(2));
        assert_eq!(iterator.next_back(), Some(3));
        assert_eq!(iterator.next_back(), None);
    }
}
