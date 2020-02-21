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
        List {
            head: self.head.as_ref().and_then(|node| node.next.clone())
        }
    }

    pub fn drop(&self, mut n: usize) -> Self {
        let mut current = &self.head;
        while n > 0 {
            match *current {
                Some(ref node) => current = &node.next,
                _ => break
            }
            n -= 1;
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
        let mut list = List::empty();
        let mut current = &self.head;
        while let Some(ref node) = *current {
            current = &node.next;
            list = list.append(node.item);
        }
        list
    }

    pub fn take(&self, mut n: usize) -> Self {
        let mut list = List::empty();
        let mut current = &self.head;
        while n > 0 {
            match *current {
                Some(ref node) => {
                    current = &node.next;
                    list = list.append(node.item);
                }
                _ => break
            }
            n -= 1;
        }
        list.reverse()
    }

    pub fn take_while<P: Fn(T) -> bool>(&self, predicate: P) -> Self {
        let mut list = List::empty();
        let mut current = &self.head;
        loop {
            match *current {
                Some(ref node) if predicate(node.item) => {
                    current = &node.next;
                    list = list.append(node.item);
                }
                _ => break
            }
        }
        list.reverse()
    }

    pub fn map<R: Copy, M: Fn(T) -> R>(&self, map: M) -> List<R> {
        let mut list = List::empty();
        let mut current = &self.head;
        while let Some(ref node) = *current {
            list = list.append(map(node.item));
            current = &node.next;
        }
        list.reverse()
    }
}

impl<T: Copy> From<Vec<T>> for List<T> {
    fn from(items: Vec<T>) -> Self {
        let mut list = List::empty();
        for item in items {
            list = list.append(item);
        }
        list
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_an_empty_list() {
        let list: List<i32> = List::empty();

        assert_eq!(list.head(), None);
    }

    #[test]
    fn head_of_a_nonempty_list() {
        assert_eq!(List::from(vec![1, 2, 3]).head(), Some(&3));
    }

    #[test]
    fn tail_of_a_list() {
        assert_eq!(List::from(vec![1, 2, 3]).tail(), List::from(vec![1, 2]));
    }

    #[test]
    fn drops_from_a_list() {
        assert_eq!(List::from(vec![1, 2, 3, 4, 5]).drop(2), List::from(vec![1, 2, 3]));
    }

    #[test]
    fn drops_from_a_list_by_predicate() {
        assert_eq!(List::from(vec![1, 2, 3, 4, 5]).drop_while(|item| item > 2), List::from(vec![1, 2]));
    }

    #[test]
    fn reverses_a_list() {
        assert_eq!(List::from(vec![1, 2, 3, 4]).reverse(), List::from(vec![4, 3, 2, 1]));
    }

    #[test]
    fn takes_from_a_list() {
        assert_eq!(List::from(vec![1, 2, 3, 4, 5]).take(2), List::from(vec![4, 5]));
    }

    #[test]
    fn takes_from_a_list_by_predicate() {
        assert_eq!(List::from(vec![1, 2, 3, 4, 5]).take_while(|item| item > 1), List::from(vec![2, 3, 4, 5]));
    }

    #[test]
    fn maps_over_a_list() {
        assert_eq!(List::from(vec![1, 2, 3, 4, 5]).map(|item| item * 3), List::from(vec![3, 6, 9, 12, 15]));
    }
}