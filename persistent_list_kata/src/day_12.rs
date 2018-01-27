use std::ops::Add;
use std::iter::FromIterator;

#[derive(PartialEq, Debug)]
pub enum List<T> {
    Empty,
    Cons(T, Box<List<T>>),
}

impl <T> List<T> {
    pub fn empty() -> Self {
        List::Empty
    }

    pub fn head(&self) -> Result<&T, EmptyListError> {
        match &self {
            List::Empty => Err(EmptyListError),
            List::Cons(head, _) => Ok(head)
        }
    }

    pub fn tail(self) -> Result<Self, EmptyListError> {
        match self {
            List::Empty => Err(EmptyListError),
            List::Cons(_, tail) => Ok(*tail)
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

    fn add(self, other: Self) -> Self::Output {
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
            list = list + item;
        }
        list
    }
}

impl <T: ToString> ToString for List<T> {
    fn to_string(&self) -> String {
        fn from_str(s: &str) -> String {
            s.to_owned()
        }

        fn from_tuple<E: ToString>(head: &E, tail: &Box<List<E>>) -> String {
            head.to_string() + internal(tail).as_str()
        }

        fn internal<E: ToString>(list: &List<E>) -> String {
            match &list {
                List::Empty => "".to_owned(),
                List::Cons(head, tail) => from_str(", ") + from_tuple(head, tail).as_str()
            }
        }

        match &self {
            List::Empty => "[]".to_owned(),
            List::Cons(head, tail) => "[".to_owned() + from_tuple(head, tail).as_str() + "]"
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct EmptyListError;

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

        assert_eq!(empty.head(), Err(EmptyListError))
    }

    #[test]
    fn head_of_nonempty_list() {
        let list = List::from_iter(vec![1, 2, 3]);

        assert_eq!(list.head(), Ok(&3));
    }

    #[test]
    fn tail_of_empty_list() {
        let empty: List<i32> = List::empty();

        assert_eq!(empty.tail(), Err(EmptyListError));
    }

    #[test]
    fn tail_of_nonempty_list() {
        let list = List::from_iter(vec![1, 2, 3]);

        assert_eq!(list.tail(), Ok(List::from_iter(vec![1, 2])));
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
        assert_eq!(
            List::from_iter(vec![4, 5, 6]) + List::from_iter(vec![1, 2, 3]),
            List::from_iter(vec![1, 2, 3, 4, 5, 6])
        );
    }
}
