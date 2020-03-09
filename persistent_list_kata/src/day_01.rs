use std::rc::Rc;

type Link<T> = Option<Rc<Node<T>>>;

#[derive(PartialEq, Debug)]
struct Node<T> {
    item: T,
    next: Link<T>
}

impl<T> Node<T> {
    fn new(item: T, next: Link<T>) -> Link<T> {
        Some(Rc::new(Node { item, next }))
    }
}

#[derive(PartialEq, Debug)]
pub struct List<T> {
    head: Link<T>
}

impl<T> List<T> {
    pub fn empty() -> Self {
        Self { head: None }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.item)
    }

    pub fn append(&mut self, item: T) -> Self {
        List { head: Node::new(item, self.head.clone()) }
    }

    pub fn tail(&self) -> Self {
        List { head: self.head.as_ref().and_then(|node| node.next.clone())}
    }
}

impl<T> AsRef<Self> for List<T> {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<'l, T> IntoIterator for &'l List<T> {
    type Item = &'l T;
    type IntoIter = ListIter<'l, T>;

    fn into_iter(self) -> Self::IntoIter {
        ListIter { node: self.head.as_ref().map(|node| &**node) }
    }
}

pub struct ListIter<'l, T> {
    node: Option<&'l Node<T>>
}

impl<'l, T> Iterator for ListIter<'l, T> {
    type Item = &'l T;

    fn next(&mut self) -> Option<Self::Item> {
        self.node.take().map(|node| {
            self.node = node.next.as_ref().map(|node| &**node);
            &node.item
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn head_of_empty_list() {
        let list: List<i32> = List::empty();

        assert_eq!(list.head(), None);
    }

    #[test]
    fn append_item_to_list() {
        let mut list = List::empty();

        list = list.append(1);

        assert_eq!(list.head(), Some(&1));
    }

    #[test]
    fn tail_of_empty_list() {
        let list: List<i32> = List::empty();

        assert_eq!(list.tail(), List::empty());
    }

    #[test]
    fn append_many_items() {
        let mut list = List::empty();

        list = list.append(1);
        list = list.append(2);
        list = list.append(3);

        assert_eq!(list.head(), Some(&3));
        assert_eq!(list.tail(), List::empty().append(1).append(2));
    }

    #[test]
    fn iterator() {
        let list = List::empty().append(1).append(2).append(3);

        let mut iter = list.into_iter();

        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }
}
