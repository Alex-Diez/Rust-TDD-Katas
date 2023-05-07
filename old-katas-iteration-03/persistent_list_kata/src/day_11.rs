use std::rc::Rc;

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    item: T,
    next: Link<T>,
}

impl<T> Node<T> {
    fn new(item: T, next: Link<T>) -> Link<T> {
        Some(Rc::new(Node { item, next }))
    }
}

pub struct List<T> {
    head: Link<T>
}

impl<T> List<T> {
    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.item)
    }

    pub fn append(&self, item: T) -> Self {
        Self { head: Node::new(item, self.head.clone()) }
    }

    pub fn tail(&self) -> Self {
        Self { head: self.head.as_ref().and_then(|node| node.next.clone()) }
    }
}

impl<T> Default for List<T> {
    fn default() -> Self {
        Self { head: None }
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

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut link = self.head.take();

        while let Some(mut node) = link.and_then(|node| Rc::try_unwrap(node).ok()) {
            link = node.next.take();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_list_head() {
        let list: List<i32> = List::default();

        assert_eq!(list.head(), None);
    }

    #[test]
    fn append_single_item() {
        let list = List::default().append(1);

        assert_eq!(list.head(), Some(&1));
    }

    #[test]
    fn append_many_items() {
        let list = List::default().append(1).append(2).append(3);

        assert_eq!(list.head(), Some(&3));
        assert_eq!(list.tail().head(), Some(&2));
        assert_eq!(list.tail().tail().head(), Some(&1));
    }

    #[test]
    fn ref_iterator() {
        let list = List::default().append(1).append(2).append(3);

        let mut iter = list.into_iter();

        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }
}
