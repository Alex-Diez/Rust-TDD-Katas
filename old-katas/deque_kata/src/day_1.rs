use std::rc::Rc;
use std::cell::{RefCell, Ref, RefMut};

#[derive(Debug)]
struct Node {
    item: i32,
    next: Link,
    prev: Link
}

impl Node {
    fn new(item: i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            item: item,
            next: None,
            prev: None
        }))
    }
}

type Link = Option<Rc<RefCell<Node>>>;

#[derive(Debug)]
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
        self.head.as_ref().map(|node| Ref::map(node.borrow(), |node| &node.item))
    }

    pub fn peek_front_mut(&mut self) -> Option<RefMut<i32>> {
        self.head.as_mut().map(|node| RefMut::map(node.borrow_mut(), |node| &mut node.item))
    }

    pub fn peek_back(&self) -> Option<Ref<i32>> {
        self.tail.as_ref().map(|node| Ref::map(node.borrow(), |node| &node.item))
    }

    pub fn peek_back_mut(&mut self) -> Option<RefMut<i32>> {
        self.tail.as_mut().map(|node| RefMut::map(node.borrow_mut(), |node| &mut node.item))
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
    fn next_back(&mut self) -> Option<i32> {
        self.deque.pop_back()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn push_front_many(deque: &mut Deque, times: i32) {
        for item in 1..times + 1 {
            deque.push_front(item);
        }
    }

    fn push_back_many(deque: &mut Deque, times: i32) {
        for item in 1..times + 1 {
            deque.push_back(item);
        }
    }

    #[test]
    fn create_an_empty_deque() {
        let mut deque = Deque::new();

        assert_eq!(deque.pop_front(), None);
    }

    #[test]
    fn pushes_one_element_to_deque_in_front() {
        let mut deque = Deque::new();

        push_front_many(&mut deque, 1);

        assert_eq!(deque.pop_front(), Some(1));
        assert_eq!(deque.pop_front(), None);
    }

    #[test]
    fn pushes_three_elements_to_deque_in_front() {
        let mut deque = Deque::new();

        push_front_many(&mut deque, 3);

        assert_eq!(deque.pop_front(), Some(3));
        assert_eq!(deque.pop_front(), Some(2));
        assert_eq!(deque.pop_front(), Some(1));
        assert_eq!(deque.pop_front(), None);
    }

    #[test]
    fn pops_from_back() {
        let mut deque = Deque::new();

        push_front_many(&mut deque, 1);

        assert_eq!(deque.pop_back(), Some(1));
        assert_eq!(deque.pop_back(), None);
    }

    #[test]
    fn pops_many_from_back() {
        let mut deque = Deque::new();

        push_front_many(&mut deque, 3);

        assert_eq!(deque.pop_back(), Some(1));
        assert_eq!(deque.pop_back(), Some(2));
        assert_eq!(deque.pop_back(), Some(3));
        assert_eq!(deque.pop_back(), None);
    }

    #[test]
    fn pushes_three_elements_back_to_deque() {
        let mut deque = Deque::new();

        push_back_many(&mut deque, 3);

        assert_eq!(deque.pop_front(), Some(1));
        assert_eq!(deque.pop_front(), Some(2));
        assert_eq!(deque.pop_front(), Some(3));
        assert_eq!(deque.pop_front(), None);
    }

    #[test]
    fn peeks_front_item_from_deque() {
        let mut deque = Deque::new();

        push_front_many(&mut deque, 3);

        assert_eq!(&*deque.peek_front().unwrap(), &3);
        assert_eq!(&*deque.peek_front().unwrap(), &3);
    }

    #[test]
    fn peeks_mut_front_item_from_deque() {
        let mut deque = Deque::new();

        push_front_many(&mut deque, 3);

        assert_eq!(&*deque.peek_front_mut().unwrap(), &mut 3);
        assert_eq!(&*deque.peek_front_mut().unwrap(), &mut 3);
    }

    #[test]
    fn peeks_back_item_from_a_deque() {
        let mut deque = Deque::new();

        push_back_many(&mut deque, 3);

        assert_eq!(&*deque.peek_back().unwrap(), &3);
        assert_eq!(&*deque.peek_back().unwrap(), &3);
    }

    #[test]
    fn peeks_mut_back_from_a_deque() {
        let mut deque = Deque::new();

        push_back_many(&mut deque, 3);

        assert_eq!(&*deque.peek_back_mut().unwrap(), &mut 3);
        assert_eq!(&*deque.peek_back_mut().unwrap(), &mut 3);
    }

    #[test]
    fn creates_iterator_from_a_deque() {
        let mut deque = Deque::new();

        push_front_many(&mut deque, 3);

        let mut iterator = deque.into_iter();

        assert_eq!(iterator.next(), Some(3));
        assert_eq!(iterator.next(), Some(2));
        assert_eq!(iterator.next(), Some(1));
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn creates_double_ended_iterator_from_a_deque() {
        let mut deque = Deque::new();

        push_back_many(&mut deque, 3);

        let mut double_ended = deque.into_iter();

        assert_eq!(double_ended.next_back(), Some(3));
        assert_eq!(double_ended.next_back(), Some(2));
        assert_eq!(double_ended.next_back(), Some(1));
        assert_eq!(double_ended.next_back(), None);
    }
}
