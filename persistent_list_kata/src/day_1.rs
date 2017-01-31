use std::rc::Rc;

#[derive(Debug, PartialEq)]
struct Node {
    item: i32,
    next: Link
}

type Link = Option<Rc<Node>>;

#[derive(Debug, PartialEq)]
pub struct List {
    head: Link
}

impl List {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn append(&self, item: i32) -> Self {
        List {
            head: Some(Rc::new(Node {
                item: item,
                next: self.head.clone()
            }))
        }
    }

    pub fn head(&self) -> Option<&i32> {
        self.head.as_ref().map(|node| &node.item)
    }

    pub fn tail(&self) -> Self {
        List { head: self.head.as_ref().and_then(|node| node.next.clone()) }
    }

    pub fn drop(&self, mut n: usize) -> Self {
        let mut current = &self.head;
        while let &Some(ref node) = current {
            if n == 0 {
                break;
            } else {
                n -= 1;
                current = &node.next;
            }
        }
        List { head: current.clone() }
    }

    pub fn drop_while<F>(&self, predicate: F) -> Self where F: Fn(i32) -> bool {
        let mut current = &self.head;
        while let &Some(ref node) = current {
            if predicate(node.item) {
                current = &node.next;
            } else {
                break;
            }
        }
        List { head: current.clone() }
    }

    pub fn take(&self, mut n: usize) -> List {
        let mut ret = List::new();
        let mut current = &self.head;
        while let &Some(ref node) = current {
            if n == 0 {
                break;
            } else {
                n -= 1;
                ret = ret.append(node.item);
                current = &node.next;
            }
        }
        ret.reverse()
    }

    pub fn reverse(&self) -> List {
        let mut ret = List::new();
        let mut current = &self.head;
        while let &Some(ref node) = current {
            ret = ret.append(node.item);
            current = &node.next;
        }
        ret
    }
}

impl<'i> IntoIterator for &'i List {
    type Item = &'i i32;
    type IntoIter = ListIterator<'i>;

    fn into_iter(self) -> Self::IntoIter {
        ListIterator { next: self.head.as_ref().map(|node| &**node) }
    }
}

pub struct ListIterator<'i> {
    next: Option<&'i Node>
}

impl<'i> Iterator for ListIterator<'i> {
    type Item = &'i i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.item
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn list_size_of(size: i32) -> List {
        let mut list = List::new();
        for e in 1..size + 1 {
            list = list.append(e);
        }
        list
    }

    #[test]
    fn creates_an_empty_list() {
        assert_eq!(list_size_of(0).head(), None);
    }

    #[test]
    fn appends_to_a_list() {
        let list = list_size_of(1);

        assert_eq!(list.head(), Some(&1));
    }

    #[test]
    fn tail_of_an_empty_list() {
        assert_eq!(list_size_of(0).tail(), List::new());
    }

    #[test]
    fn tail_of_a_nonempty_list() {
        assert_eq!(list_size_of(3).tail(), list_size_of(2));
    }

    #[test]
    fn ref_iterator_over_a_list() {
        let list = list_size_of(3);

        let mut iterator = list.into_iter();

        assert_eq!(iterator.next(), Some(&3));
        assert_eq!(iterator.next(), Some(&2));
        assert_eq!(iterator.next(), Some(&1));
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn drops_nothing_from_an_empty_list() {
        assert_eq!(list_size_of(0).drop(10), list_size_of(0));
    }

    #[test]
    fn drop_two_items_from_three_elements_list() {
        assert_eq!(list_size_of(3).drop(2), list_size_of(1));
    }

    #[test]
    fn drops_nothing_from_an_empty_list_by_predicate() {
        assert_eq!(list_size_of(0).drop_while(|e| e > 10), List::new());
    }

    #[test]
    fn drops_while_items_match_predicate() {
        assert_eq!(list_size_of(5).drop_while(|item| item > 2), list_size_of(2));
    }

    #[test]
    fn takes_nothing_from_an_empty_list() {
        assert_eq!(list_size_of(0).take(10), list_size_of(0));
    }

    #[test]
    fn reverses_list() {
        let list = list_size_of(3);

        assert_eq!(list.reverse(), List::new().append(3).append(2).append(1));
    }

    #[test]
    fn takes_three_items_from_a_list() {
        let list = list_size_of(5);

        assert_eq!(list.take(3), List::new().append(3).append(4).append(5));
    }
}