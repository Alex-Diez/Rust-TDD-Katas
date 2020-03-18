use std::rc::Rc;

struct Node {
    item: i32,
    next: Option<Rc<Node>>
}

pub struct List {
    head: Option<Rc<Node>>
}

impl List {
    pub fn head(&self) -> Option<&i32> {
        self.head.as_ref().map(|node| &node.item)
    }

    pub fn append(&self, item: i32) -> Self {
        Self { head: Some(Rc::new(Node{item, next: self.head.clone()}))}
    }

    pub fn tail(&self) -> Self {
        Self { head: self.head.as_ref().and_then(|node| node.next.clone())}
    }
}

impl Default for List {
    fn default() -> Self {
        Self { head: None }
    }
}

impl<'l> IntoIterator for &'l List {
    type Item = &'l i32;
    type IntoIter = ListIter<'l>;

    fn into_iter(self) -> Self::IntoIter {
        ListIter { node: self.head.as_ref().map(|node| &**node)}
    }
}

pub struct ListIter<'l> {
    node: Option<&'l Node>
}

impl<'l> Iterator for ListIter<'l> {
    type Item = &'l i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.node.take().map(|node| {
            self.node = node.next.as_ref().map(|node| &**node);
            &node.item
        })
    }
}

impl Drop for List {
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
        let list = List::default();

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

    #[test]
    fn middle_list_drop() {
        let middle_list = List::default().append(1).append(2);

        let list = middle_list.append(3);

        let mut iter = list.into_iter();

        drop(middle_list);

        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }
}
