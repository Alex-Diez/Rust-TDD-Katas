use std::rc::Rc;

#[derive(Debug, PartialEq)]
struct Node<T> {
    item: T,
    next: Link<T>
}

type Link<T> = Option<Rc<Node<T>>>;

#[derive(Debug, PartialEq)]
pub struct List<T> {
    head: Link<T>
}

impl<T: Copy> List<T> {
    pub fn empty() -> Self {
        List { head: None }
    }

    pub fn single(item: T) -> Self {
        List {
            head: Some(Rc::new(Node {
                item: item,
                next: None
            }))
        }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.item)
    }

    pub fn append(&self, item: T) -> Self {
        List {
            head: Some(Rc::new(Node {
                item: item,
                next: self.head.clone()
            }))
        }
    }

    pub fn tail(&self) -> Self {
        List { head: self.head.as_ref().and_then(|node| node.next.clone()) }
    }

    pub fn drop(&self, mut n: usize) -> Self {
        let mut current = &self.head;
        while n > 0 {
            match *current {
                Some(ref node) => {
                    n -= 1;
                    current = &node.next;
                },
                _ => break
            }
        }
        List { head: current.clone() }
    }

    pub fn drop_while<P: Fn(T) -> bool>(&self, predicate: P) -> Self {
        let mut current = &self.head;
        loop {
            match *current {
                Some(ref node) if predicate(node.item) => current = &node.next,
                _ => break
            }
        }
        List { head: current.clone() }
    }

    pub fn reverse(&self) -> Self {
        let mut current = &self.head;
        let mut head = None;
        while let Some(ref node) = *current {
            head = Some(Rc::new(Node {
                item: node.item,
                next: head.take()
            }));
            current = &node.next;
        }
        List { head: head }
    }

    pub fn take(&self, mut n: usize) -> Self {
        let mut current = &self.head;
        let mut head = None;
        loop {
            match *current {
                Some(ref node) if n > 0 => {
                    n -= 1;
                    head = Some(Rc::new(Node {
                        item: node.item,
                        next: head.take()
                    }));
                    current = &node.next;
                },
                _ => break
            }
        }
        List { head: head }.reverse()
    }

    pub fn take_while<P: Fn(T) -> bool>(&self, predicate: P) -> Self {
        let mut current = &self.head;
        let mut head = None;
        loop {
            match *current {
                Some(ref node) if predicate(node.item) => {
                    head = Some(Rc::new(Node {
                        item: node.item,
                        next: head.take()
                    }));
                    current = &node.next;
                },
                _ => break
            }
        }
        List { head: head }.reverse()
    }

    pub fn map<R: Copy, M: Fn(T) -> R>(&self, map: M) -> List<R> {
        let mut current = &self.head;
        let mut head = None;
        while let Some(ref node) = *current {
            head = Some(Rc::new(Node {
                item: map(node.item),
                next: head.take()
            }));
            current = &node.next;
        }
        List { head: head }.reverse()
    }
}

impl<'i, T> IntoIterator for &'i List<T> {
    type Item = &'i T;
    type IntoIter = ListIterator<'i, T>;

    fn into_iter(self) -> Self::IntoIter {
        ListIterator { next: self.head.as_ref().map(|node| &**node) }
    }
}

pub struct ListIterator<'i, T: 'i> {
    next: Option<&'i Node<T>>
}

impl<'i, T> Iterator for ListIterator<'i, T> {
    type Item = &'i T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.item
        })
    }
}

impl<T: Copy> From<Vec<T>> for List<T> {
    fn from(items: Vec<T>) -> Self {
        let mut head = None;
        for item in items {
            head = Some(Rc::new(Node {
                item: item,
                next: head.take()
            }));
        }
        List { head: head }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_an_empty_list() {
        let empty: List<i32> = List::empty();
        assert_eq!(empty.head(), None);
    }

    #[test]
    fn takes_head_of_a_nonempty_list() {
        assert_eq!(List::from(vec![1, 2]).head(), Some(&2));
    }

    #[test]
    fn tail_of_an_empty_list() {
        let empty: List<i32> = List::empty();
        assert_eq!(empty.tail(), List::empty());
    }

    #[test]
    fn tail_of_a_nonempty_list() {
        assert_eq!(List::from(vec![1, 2, 3]).tail(), List::from(vec![1, 2]));
    }

    #[test]
    fn ref_iterator_over_a_list() {
        let list = List::from(vec![1, 2, 3]);

        let mut iterator = list.into_iter();

        assert_eq!(iterator.next(), Some(&3));
        assert_eq!(iterator.next(), Some(&2));
        assert_eq!(iterator.next(), Some(&1));
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn drops_from_a_list() {
        assert_eq!(List::from(vec![1, 2, 3]).drop(2), List::single(1));
    }

    #[test]
    fn drops_by_predicate_from_a_list() {
        assert_eq!(List::from(vec![1, 2, 3]).drop_while(|item| item > 2), List::from(vec![1, 2]));
    }

    #[test]
    fn reverse_a_list() {
        assert_eq!(List::from(vec![1, 2, 3]).reverse(), List::from(vec![3, 2, 1]));
    }

    #[test]
    fn takes_from_a_list() {
        assert_eq!(List::from(vec![1, 2, 3]).take(2), List::from(vec![2, 3]));
    }

    #[test]
    fn takes_from_a_list_by_predicate() {
        assert_eq!(List::from(vec![1, 2, 3]).take_while(|item| item > 1), List::from(vec![2, 3]));
    }

    #[test]
    fn maps_list() {
        assert_eq!(List::from(vec![1, 2, 3]).map(|item| item * 2), List::from(vec![2, 4, 6]));
    }
}