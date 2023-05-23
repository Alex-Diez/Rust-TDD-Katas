use std::rc::Rc;

type Link = Option<Rc<Node>>;

#[derive(PartialEq, Debug)]
struct List {
    node: Link,
}

#[derive(PartialEq, Debug)]
struct Node {
    item: i32,
    next: Link,
}

impl List {
    fn empty() -> List {
        List { node: None }
    }

    fn head(&self) -> Option<&i32> {
        self.node.as_ref().map(|node| &node.item)
    }

    fn tail(&self) -> List {
        List {
            node: self.node.as_ref().and_then(|node| node.next.clone()),
        }
    }

    fn prepend(&self, item: i32) -> List {
        List {
            node: Some(Rc::new(Node {
                item,
                next: self.node.clone(),
            })),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn head_of_empty_list() {
        let empty = List::empty();

        assert_eq!(empty.head(), None);
    }

    #[test]
    fn tail_of_empty_list() {
        let empty = List::empty();

        assert_eq!(empty.tail(), List::empty());
    }

    #[test]
    fn head_of_single_item_list() {
        let single = List::empty().prepend(1);

        assert_eq!(single.head(), Some(&1));
    }

    #[test]
    fn tail_of_single_item_list() {
        let single = List::empty().prepend(1);

        assert_eq!(single.tail(), List::empty());
    }

    #[test]
    fn head_of_multiple_items_list() {
        let many = List::empty().prepend(1).prepend(2).prepend(3);

        assert_eq!(many.head(), Some(&3));
    }

    #[test]
    fn tail_of_multiple_items_list() {
        let many = List::empty().prepend(1).prepend(2).prepend(3);

        assert_eq!(many.tail(), List::empty().prepend(1).prepend(2));
    }
}
