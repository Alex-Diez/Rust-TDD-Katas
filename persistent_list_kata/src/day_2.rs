use std::rc::Rc;

#[derive(Debug, PartialEq)]
struct Node<T> {
    item: T,
    next: Link<T>
}

impl<T: Copy> Node<T> {
    fn create_node(&self, mut next: Link<T>) -> Link<T> {
        Some(Rc::new(Node {
            item: self.item,
            next: next.take()
        }))
    }

    fn map_node<R, M: Fn(T) -> R>(&self, mut next: Link<R>, map: &M) -> Link<R> {
        Some(Rc::new(Node {
            item: map(self.item),
            next: next.take()
        }))
    }
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

    pub fn append(self, item: T) -> Self {
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
        loop {
            match *current {
                Some(ref node) if n > 0 => {
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
            head = node.create_node(head);
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
                    head = node.create_node(head);
                    current = &node.next;
                    n -= 1;
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
                    head = node.create_node(head);
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
            head = node.map_node(head, &map);
            current = &node.next;
        }
        List { head: head }.reverse()
    }
}

impl<T> From<Vec<T>> for List<T> {
    fn from(items: Vec<T>) -> Self {
        let mut head = None;
        for item in items {
            head = Some(Rc::new(Node {
                item: item,
                next: head.take()
            }))
        }
        List { head: head }
    }
}

impl<'l, T> IntoIterator for &'l List<T> {
    type Item = &'l T;
    type IntoIter = ListIterator<'l, T>;

    fn into_iter(self) -> Self::IntoIter {
        ListIterator { next: self.head.as_ref().map(|node| &**node) }
    }
}

pub struct ListIterator<'l, T: 'l> {
    next: Option<&'l Node<T>>
}

impl<'l, T> Iterator for ListIterator<'l, T> {
    type Item = &'l T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.item
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_an_empty_list() {
        let empty: List<i32> = List::empty();
        assert_eq! (empty.head(), None);
    }

    #[test]
    fn appends_to_a_list() {
        assert_eq! (List::from(vec![1, 2, 3]).head(), Some(&3));
    }

    #[test]
    fn tail_of_an_empty_list() {
        let empty: List<i32> = List::empty();
        assert_eq! (empty.tail(), List::empty());
    }

    #[test]
    fn tail_of_a_nonempty_list() {
        assert_eq! (List::from(vec![1, 2, 3]).tail(), List::from(vec![1, 2]));
    }

    #[test]
    fn ref_iterator_over_a_list() {
        let list = List::from(vec![1, 2, 3]);

        let mut iterator = list.into_iter();

        assert_eq!(iterator.next(), Some(&3));
        assert_eq! (iterator.next(), Some(&2));
        assert_eq! (iterator.next(), Some(&1));
        assert_eq! (iterator.next(), None);
    }

    #[test]
    fn drops_nothing_from_an_empty_list() {
        let empty: List<i32> = List::empty();
        assert_eq! (empty.drop(10), List::empty());
    }

    #[test]
    fn drops_two_items_form_a_list() {
        assert_eq! (List::from(vec![1, 2, 3]).drop(2), List::single(1));
    }

    #[test]
    fn drop_while_from_a_nonempty_list() {
        assert_eq! (List::from(vec![1, 2, 3]).drop_while(|item| item > 1), List::single(1));
    }

    #[test]
    fn reverses_a_list() {
        assert_eq! (List::from(vec![1, 2, 3]).reverse(), List::from(vec![3, 2, 1]));
    }

    #[test]
    fn takes_two_elements_from_a_list() {
        assert_eq! (List::from(vec![1, 2, 3]).take(2), List::from(vec![2, 3]));
    }

    #[test]
    fn takes_while_item_matches_predicate() {
        assert_eq! (List::from(vec![1, 2, 3, 4, 5, 6]).take_while(|item| item > 3), List::from(vec![4, 5, 6]));
    }

    #[test]
    fn maps_list() {
        assert_eq! (List::from(vec![1, 2, 3, 4, 5, 6]).map(|item| item * 2), List::from(vec![2, 4, 6, 8, 10, 12]));
    }
}