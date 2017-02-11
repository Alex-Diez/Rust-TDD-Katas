use std::rc::Rc;
use std::cell::{RefCell, Ref, RefMut};

struct Node<T> {
    item: T,
    next: Link<T>,
    prev: Link<T>
}

impl<T> Node<T> {
    pub fn new(item: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node{
            item: item,
            next: None,
            prev: None
        }))
    }
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

pub struct Deque<T> {
    head: Link<T>,
    tail: Link<T>
}

impl<T> Deque<T> {
    pub fn new() -> Self {
        Deque { head: None, tail: None }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev.take();
                    self.head = Some(new_head)
                }
                None => { self.tail.take(); }
            }
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().item
        })
    }

    pub fn push_front(&mut self, item: T) {
        let new_head = Node::new(item);
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head);
                self.head = Some(new_head)
            }
            None => {
                self.head = Some(new_head.clone());
                self.tail = Some(new_head);
            }
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            match old_tail.borrow_mut().prev.take() {
                Some(new_tail) => {
                    new_tail.borrow_mut().next.take();
                    self.tail = Some(new_tail)
                }
                None => { self.head.take(); }
            }
            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().item
        })
    }

    pub fn push_back(&mut self, item: T) {
        let new_tail = Node::new(item);
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(old_tail);
                self.tail = Some(new_tail)
            }
            None => {
                self.head = Some(new_tail.clone());
                self.tail = Some(new_tail);
            }
        }
    }

    pub fn peek_front(&self) -> Option<Ref<T>> {
        self.head.as_ref().map(|node| {
            Ref::map(node.borrow(), |node| &node.item)
        })
    }

    pub fn peek_front_mut(&mut self) -> Option<RefMut<T>> {
        self.head.as_mut().map(|node| {
            RefMut::map(node.borrow_mut(), |node| &mut node.item)
        })
    }

    pub fn peek_back(&self) -> Option<Ref<T>> {
        self.tail.as_ref().map(|node| {
            Ref::map(node.borrow(), |node| &node.item)
        })
    }

    pub fn peek_back_mut(&mut self) -> Option<RefMut<T>> {
        self.tail.as_mut().map(|node| {
            RefMut::map(node.borrow_mut(), |node| &mut node.item)
        })
    }
}

impl<T> IntoIterator for Deque<T> {
    type Item = T;
    type IntoIter = DequeIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        DequeIterator { deque: self }
    }
}

pub struct DequeIterator<T> {
    deque: Deque<T>
}

impl<T> Iterator for DequeIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.deque.pop_front()
    }
}

impl<T> DoubleEndedIterator for DequeIterator<T> {
    fn next_back(&mut self) -> Option<T> {
        self.deque.pop_back()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn push_many_in_front(deque: &mut Deque<i32>, times: i32) {
        for item in 1..times + 1 {
            deque.push_front(item);
        }
    }

    #[test]
    fn creates_an_empty_deque() {
        let mut deque: Deque<i32> = Deque::new();

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
    fn pops_item_from_front_of_deque() {
        let mut deque = Deque::new();

        push_many_in_front(&mut deque, 3);

        assert_eq!(deque.pop_back(), Some(1));
        assert_eq!(deque.pop_back(), Some(2));
        assert_eq!(deque.pop_back(), Some(3));
        assert_eq!(deque.pop_back(), None);
    }
    
    #[test]
    fn pushes_three_items_in_back_of_deque() {
        let mut deque = Deque::new();
        
        deque.push_back(1);
        deque.push_back(2);
        deque.push_back(3);

        assert_eq!(deque.pop_back(), Some(3));
        assert_eq!(deque.pop_back(), Some(2));
        assert_eq!(deque.pop_back(), Some(1));
        assert_eq!(deque.pop_back(), None);
    }

    #[test]
    fn peeks_front_item() {
        let mut deque = Deque::new();

        push_many_in_front(&mut deque, 3);

        assert_eq!(&*deque.peek_front().unwrap(), &3);
        assert_eq!(&*deque.peek_front().unwrap(), &3);
    }

    #[test]
    fn peeks_mut_front_item() {
        let mut deque = Deque::new();

        push_many_in_front(&mut deque, 3);

        assert_eq!(&*deque.peek_front_mut().unwrap(), &mut 3);
        assert_eq!(&*deque.peek_front_mut().unwrap(), &mut 3);
    }

    #[test]
    fn peeks_back_item() {
        let mut deque = Deque::new();

        push_many_in_front(&mut deque, 3);

        assert_eq!(&*deque.peek_back().unwrap(), &1);
        assert_eq!(&*deque.peek_back().unwrap(), &1);
    }

    #[test]
    fn peeks_mut_back_item() {
        let mut deque = Deque::new();

        push_many_in_front(&mut deque, 3);

        assert_eq!(&*deque.peek_back_mut().unwrap(), &mut 1);
        assert_eq!(&*deque.peek_back_mut().unwrap(), &mut 1);
    }

    #[test]
    fn creates_iterator_over_a_deque() {
        let mut deque = Deque::new();

        push_many_in_front(&mut deque, 3);

        let mut iterator = deque.into_iter();

        assert_eq!(iterator.next(), Some(3));
        assert_eq!(iterator.next(), Some(2));
        assert_eq!(iterator.next(), Some(1));
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn creates_double_ended_iterator() {
        let mut deque = Deque::new();

        push_many_in_front(&mut deque, 3);

        let mut iterator = deque.into_iter();

        assert_eq!(iterator.next_back(), Some(1));
        assert_eq!(iterator.next_back(), Some(2));
        assert_eq!(iterator.next_back(), Some(3));
        assert_eq!(iterator.next_back(), None);
    }
}
