use std::fmt;
use std::ops::Add;

#[derive(PartialEq)]
pub enum List {
    Empty,
    Cons(i32, Box<List>),
}

impl List {
    pub fn empty() -> Self {
        List::Empty
    }

    pub fn head(&self) -> Option<&i32> {
        match self {
            &List::Empty => None,
            &List::Cons(ref head, _) => Some(&head)
        }
    }

    pub fn tail(self) -> Self {
        match self {
            List::Empty => self,
            List::Cons(_, tail) => *tail
        }
    }
}

impl Add<i32> for List {
    type Output = Self;

    fn add(self, item: i32) -> Self::Output {
        List::Cons(item, Box::new(self))
    }
}

impl Add<List> for List {
    type Output = Self;

    fn add(self, other: List) -> Self::Output {
        match self {
            List::Empty => other,
            List::Cons(head, tail) => (*tail + other) + head
        }
    }
}

impl fmt::Debug for List {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self.to_string())
    }
}

impl ToString for List {
    fn to_string(&self) -> String {
        fn inner(list: &List) -> String {
            match list {
                &List::Empty => "".to_owned(),
                &List::Cons(ref head, ref tail) => ", ".to_owned() + head.to_string().as_str() + inner(&*tail).as_str()
            }
        }
        match self {
            &List::Empty => "[]".to_owned(),
            &List::Cons(ref head, ref tail) => "[".to_owned() + head.to_string().as_str() + inner(&*tail).as_str() + "]"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_empty_list() {
        let empty = List::empty();

        assert_eq!(empty.to_string(), "[]");
    }

    #[test]
    fn prepend_items_to_list() {
        let list = List::empty() + 1 + 2 + 3;

        assert_eq!(list.to_string(), "[3, 2, 1]");
    }

    #[test]
    fn head_of_empty_list() {
        let empty = List::empty();
        assert_eq!(empty.head(), None);
    }

    #[test]
    fn head_of_nonempty_list() {
        let list = List::empty() + 1 + 2 + 3;
        assert_eq!(list.head(), Some(&3));
    }

    #[test]
    fn tail_of_empty_list() {
        let empty = List::empty();
        assert_eq!(empty.tail(), List::empty());
    }

    #[test]
    fn tail_of_nonempty_list() {
        let list = List::empty() + 1 + 2 + 3;
        assert_eq!(list.tail(), List::empty() + 1 + 2);
    }

    #[test]
    fn concatenation_of_two_empty_lists() {
        let empty = List::empty();
        assert_eq!(empty + List::empty(), List::empty())
    }

    #[test]
    fn concatenation_of_empty_and_nonempty_lists() {
        assert_eq!(List::empty() + (List::empty() + 1 + 2 + 3), List::empty() + 1 + 2 + 3);
        assert_eq!((List::empty() + 1 + 2 + 3) + List::empty(), List::empty() + 1 + 2 + 3);
    }

    #[test]
    fn concatenation_of_two_nonempty_lists() {
        assert_eq!((List::empty() + 4 + 5 + 6) + (List::empty() + 1 + 2 + 3), List::empty() + 1 + 2 + 3 + 4 + 5 + 6);
    }
}
