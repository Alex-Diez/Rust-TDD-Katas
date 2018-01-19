use std::fmt;
use std::iter::FromIterator;
use std::ops::Add;

#[derive(PartialEq)]
pub enum List<T> {
    Empty,
    Cons(T, Box<List<T>>)
}

impl <T> List<T> {
    pub fn empty() -> Self {
        List::Empty
    }

    pub fn head(&self) -> Option<&T> {
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

impl <T> Add<T> for List<T> {
    type Output = Self;

    fn add(self, item: T) -> Self::Output {
        List::Cons(item, Box::new(self))
    }
}

impl <T> Add<List<T>> for List<T> {
    type Output = Self;

    fn add(self, other: List<T>) -> Self::Output {
        match self {
            List::Empty => other,
            List::Cons(head, tail) => (*tail + other) + head
        }
    }
}

impl <T> FromIterator<T> for List<T> {
    fn from_iter<I: IntoIterator<Item=T>>(items: I) -> Self {
        let mut list = List::empty();
        for item in items {
            list = list + item
        }
        list
    }
}

impl <T: ToString> fmt::Debug for List<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self.to_string())
    }
}

impl <T: ToString> ToString for List<T> {
    fn to_string(&self) -> String {
        fn inner<E: ToString>(list: &List<E>) -> String {
            match list {
                &List::Empty => "".to_owned(),
                &List::Cons(ref head, ref tail) => String::from(", ") + head.to_string().as_str() + inner(&*tail).as_str()
            }
        }

        match self {
            &List::Empty => "[]".to_owned(),
            &List::Cons(ref head, ref tail) => String::from("[") + head.to_string().as_str() + inner(&*tail).as_str() + "]"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_empty_list() {
        let empty: List<i32> = List::empty();
        assert_eq!(empty.to_string(), "[]");
    }

    #[test]
    fn prepend_items_to_list() {
        let list = List::empty() + 1 + 2 + 3;

        assert_eq!(list.to_string(), "[3, 2, 1]");
    }

    #[test]
    fn head_of_empty_list() {
        let empty: List<i32> = List::empty();
        assert_eq!(empty.head(), None);
    }

    #[test]
    fn head_of_nonempty_list() {
        assert_eq!((List::from_iter(vec![1, 2, 3])).head(), Some(&3));
    }

    #[test]
    fn tail_of_empty_list() {
        let empty: List<i32> = List::empty();
        assert_eq!(empty.tail(), List::empty());
    }

    #[test]
    fn tail_of_nonempty_list() {
        assert_eq!(List::from_iter(vec![1, 2, 3]).tail(), (List::empty() + 1 + 2));
    }

    #[test]
    fn concatenation_of_two_empty_lists() {
        let empty: List<i32> = List::empty();
        assert_eq!(empty + List::empty(), List::empty());
    }

    #[test]
    fn concatenation_of_empty_and_nonempty_lists() {
        let empty: List<i32> = List::empty();
        assert_eq!(empty + List::from_iter(vec![1, 2, 3]), List::from_iter(vec![1, 2, 3]));
        assert_eq!(List::from_iter(vec![1, 2, 3]) + List::empty(), List::from_iter(vec![1, 2, 3]));
    }

    #[test]
    fn concatenation_of_two_nonempty_lists() {
        assert_eq!(List::from_iter(vec![4, 5, 6]) + List::from_iter(vec![1, 2, 3]), List::from_iter(vec![1, 2, 3, 4, 5, 6]));
    }
}
