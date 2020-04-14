use std::ptr::NonNull;

type Link = Option<NonNull<Node>>;

struct Node {
    item: i32,
    next: Link,
    prev: Link,
}

impl Node {
    fn new(item: i32) -> Box<Self> {
        Box::new(Self { item, next: None, prev: None })
    }
}

pub struct UnsafeDeque {
    head: Link,
    tail: Link,
}

impl UnsafeDeque {
    pub fn pop_front(&mut self) -> Option<i32> {
        self.head.take().map(|mut old_head| unsafe {
            match old_head.as_mut().next.take() {
                Some(mut new_head) => {
                    new_head.as_mut().prev.take();
                    self.head = Some(new_head);
                }
                None => { self.tail.take(); }
            }
            old_head.as_ref().item
        })
    }

    pub fn push_front(&mut self, item: i32) {
        if let Some(mut new_head) = NonNull::new(Box::into_raw(Node::new(item))) {
            unsafe {
                new_head.as_mut().next = self.head;
                match self.head {
                    Some(mut old_head) => old_head.as_mut().prev = Some(new_head),
                    None => self.tail = Some(new_head)
                }
                self.head = Some(new_head);
            }
        }
    }

    pub fn pop_back(&mut self) -> Option<i32> {
        self.tail.take().map(|mut old_tail| unsafe {
            match old_tail.as_mut().prev.take() {
                Some(mut new_tail) => {
                    new_tail.as_mut().next.take();
                    self.tail = Some(new_tail);
                }
                None => { self.head.take(); }
            }
            old_tail.as_ref().item
        })
    }

    pub fn push_back(&mut self, item: i32) {
        if let Some(mut new_tail) = NonNull::new(Box::into_raw(Node::new(item))) {
            unsafe {
                new_tail.as_mut().prev = self.tail;
                match self.tail {
                    Some(mut old_tail) => old_tail.as_mut().next = Some(new_tail),
                    None => self.head = Some(new_tail)
                }
                self.tail = Some(new_tail);
            }
        }
    }

    pub fn peek_front(&self) -> Option<&i32> {
        self.head.as_ref().map(|head| unsafe { &head.as_ref().item })
    }

    pub fn peek_back(&self) -> Option<&i32> {
        self.tail.as_ref().map(|tail| unsafe { &tail.as_ref().item })
    }

    pub fn peek_front_mut(&mut self) -> Option<&mut i32> {
        self.head.as_mut().map(|head| unsafe { &mut head.as_mut().item })
    }

    pub fn peek_back_mut(&mut self) -> Option<&mut i32> {
        self.tail.as_mut().map(|tail| unsafe { &mut tail.as_mut().item })
    }
}

impl Default for UnsafeDeque {
    fn default() -> Self {
        Self { head: None, tail: None }
    }
}

impl IntoIterator for UnsafeDeque {
    type Item = i32;
    type IntoIter = UnsafeDequeIter;

    fn into_iter(self) -> Self::IntoIter {
        UnsafeDequeIter { deque: self }
    }
}

pub struct UnsafeDequeIter {
    deque: UnsafeDeque
}

impl Iterator for UnsafeDequeIter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.deque.pop_front()
    }
}

impl DoubleEndedIterator for UnsafeDequeIter {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.deque.pop_back()
    }
}

impl AsRef<Self> for UnsafeDeque {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<'d> IntoIterator for &'d UnsafeDeque {
    type Item = &'d i32;
    type IntoIter = UnsafeDequeIterRef<'d>;

    fn into_iter(self) -> Self::IntoIter {
        UnsafeDequeIterRef { node: self.head.as_ref().map(|head| unsafe { &*head.as_ref() })}
    }
}

pub struct UnsafeDequeIterRef<'d> {
    node: Option<&'d Node>
}

impl<'d> Iterator for UnsafeDequeIterRef<'d> {
    type Item = &'d i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.node.take().map(|node| {
            self.node = node.next.as_ref().map(|node| unsafe { &*node.as_ref()});
            &node.item
        })
    }
}

impl AsMut<Self> for UnsafeDeque {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl<'d> IntoIterator for &'d mut UnsafeDeque {
    type Item = &'d mut i32;
    type IntoIter = UnsafeDequeIterRefMut<'d>;

    fn into_iter(self) -> Self::IntoIter {
        UnsafeDequeIterRefMut { node: self.head.as_mut().map(|node| unsafe { &mut *node.as_mut() })}
    }
}

pub struct UnsafeDequeIterRefMut<'d> {
    node: Option<&'d mut Node>
}

impl<'d> Iterator for UnsafeDequeIterRefMut<'d> {
    type Item = &'d mut i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.node.take().map(|node| {
            self.node = node.next.as_mut().map(|node| unsafe { &mut *node.as_mut() });
            &mut node.item
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pop_front_from_empty_deque() {
        let mut deque = UnsafeDeque::default();

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
    fn peek_front() {
        let mut deque = UnsafeDeque::default();

        deque.push_front(1);
        deque.push_front(2);

        assert_eq!(deque.peek_front(), Some(&2));
    }

    #[test]
    fn peek_back() {
        let mut deque = UnsafeDeque::default();

        deque.push_front(1);
        deque.push_front(2);

        assert_eq!(deque.peek_back(), Some(&1));
    }

    #[test]
    fn peek_front_mut() {
        let mut deque = UnsafeDeque::default();

        deque.push_front(2);
        deque.push_front(3);

        deque.peek_front_mut().map(|x| *x = *x * 2);

        assert_eq!(deque.peek_front(), Some(&6));
    }

    #[test]
    fn peek_back_mut() {
        let mut deque = UnsafeDeque::default();

        deque.push_front(2);
        deque.push_front(3);

        deque.peek_back_mut().map(|x| *x = *x * 2);

        assert_eq!(deque.peek_back(), Some(&4));
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

    #[test]
    fn ref_iterator() {
        let mut deque = UnsafeDeque::default();

        deque.push_front(1);
        deque.push_front(2);
        deque.push_front(3);

        let mut iter = deque.as_ref().into_iter();

        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn mut_ref_iterator() {
        let mut deque = UnsafeDeque::default();

        deque.push_front(1);
        deque.push_front(2);
        deque.push_front(3);

        let mut iter = deque.as_mut().into_iter();

        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), None);
    }
}
