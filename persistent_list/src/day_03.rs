use std::rc::Rc;

type Link = Option<Rc<Node>>;

#[derive(PartialEq, Debug)]
struct Node {
    item: i32,
    next: Link,
}

impl Node {
    fn new(item: i32, next: Link) -> Link {
        Some(Rc::new(Node { item, next }))
    }
}

#[derive(PartialEq, Debug)]
pub struct List {
    head: Link,
}

impl List {
    pub(crate) fn empty() -> List {
        List { head: None }
    }

    pub(crate) fn head(&self) -> Option<&i32> {
        self.head.as_ref().map(|node| &node.item)
    }

    pub(crate) fn tail(&self) -> List {
        List {
            head: self.head.as_ref().and_then(|node| node.next.clone()),
        }
    }

    pub(crate) fn prepend(&self, item: i32) -> List {
        List {
            head: Node::new(item, self.head.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_list_head() {
        let empty = List::empty();

        assert_eq!(empty.head(), None);
    }

    #[test]
    fn empty_list_tail() {
        let empty = List::empty();

        assert_eq!(empty.tail(), List::empty());
    }

    #[test]
    fn prepend_single_item() {
        let list = List::empty().prepend(1);

        assert_eq!(list.head(), Some(&1));
        assert_eq!(list.tail(), List::empty());
    }

    #[test]
    fn prepend_multiple_items() {
        let list = List::empty().prepend(1).prepend(2).prepend(3);

        assert_eq!(list.head(), Some(&3));
        assert_eq!(list.tail(), List::empty().prepend(1).prepend(2));
    }
}
