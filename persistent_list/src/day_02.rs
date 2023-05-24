use std::rc::Rc;

type Link<T> = Option<Rc<Node<T>>>;

#[derive(PartialEq, Debug)]
struct Node<T> {
    item: T,
    next: Link<T>,
}

impl<T> Node<T> {
    fn new(item: T, next: Link<T>) -> Link<T> {
        Some(Rc::new(Node { item, next }))
    }
}

#[derive(PartialEq, Debug)]
pub struct List<T> {
    head: Link<T>,
}

impl<T> List<T> {
    fn empty() -> List<T> {
        List { head: None }
    }

    fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.item)
    }

    fn tail(&self) -> List<T> {
        List {
            head: self.head.as_ref().and_then(|node| node.next.clone()),
        }
    }

    fn prepend(&self, item: T) -> List<T> {
        List {
            head: Node::new(item, self.head.clone()),
        }
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
    fn tail_of_empty_list() {
        let list: List<i32> = List::empty();

        assert_eq!(list.tail(), List::empty());
    }

    #[test]
    fn prepend_single_item() {
        let empty = List::empty();
        let list = empty.prepend(1);

        assert_eq!(list.head(), Some(&1));
        assert_eq!(list.tail(), empty)
    }

    #[test]
    fn prepend_many_items() {
        let list = List::empty().prepend(1).prepend(2).prepend(3);

        assert_eq!(list.head(), Some(&3));
        assert_eq!(list.tail(), List::empty().prepend(1).prepend(2));
    }
}
