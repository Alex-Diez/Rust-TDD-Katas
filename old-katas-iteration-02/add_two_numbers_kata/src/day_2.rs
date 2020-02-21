pub fn add_two_numbers(first: List, second: List) -> List {
    let val = first.val + second.val;
    if val > 9 {
        let next = match (first.next, second.next) {
            (Some(first), Some(second)) => add_two_numbers(*first, add_two_numbers(List::new(val / 10), *second)),
            (Some(first), None) => add_two_numbers(*first, List::new(val / 10)),
            (None, Some(second)) => add_two_numbers(List::new(val / 10), *second),
            (None, None) => List::new(val / 10)
        };
        List::with_next(val % 10, next)
    } else {
        match (first.next, second.next) {
            (Some(first), Some(second)) => List::with_next(val, add_two_numbers(*first, *second)),
            (_, _) => List::new(val)
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct List {
    val: u32,
    next: Option<Box<List>>,
}

impl List {
    pub fn new(val: u32) -> Self {
        List { val, next: None }
    }

    pub fn with_next(val: u32, next: Self) -> Self {
        List { val, next: Some(Box::new(next)) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn from(num: u32) -> List {
        if num > 9 {
            List::with_next(num % 10, from(num / 10))
        } else {
            List::new(num)
        }
    }

    #[test]
    fn add_two_zeros() {
        assert_eq!(add_two_numbers(List::new(0), List::new(0)), List::new(0));
    }

    #[test]
    fn add_single_digit_numbers_without_overflow() {
        assert_eq!(add_two_numbers(List::new(4), List::new(5)), List::new(9));
    }

    #[test]
    fn add_single_digit_numbers_with_overflow() {
        assert_eq!(add_two_numbers(List::new(7), List::new(8)), from(15));
    }

    #[test]
    fn add_two_digits_numbers_without_overflow() {
        assert_eq!(add_two_numbers(from(11), from(22)), from(33));
    }

    #[test]
    fn add_two_digits_numbers_with_overflow() {
        assert_eq!(add_two_numbers(from(15), from(17)), from(32));
        assert_eq!(add_two_numbers(from(99), from(17)), from(116));
    }

    #[test]
    fn add_three_digits_numbers_without_overflow() {
        assert_eq!(add_two_numbers(from(111), from(222)), from(333));
    }

    #[test]
    fn add_three_digits_numbers_with_overflow() {
        assert_eq!(add_two_numbers(from(100), from(900)), from(1000));
        assert_eq!(add_two_numbers(from(999), from(2)), from(1001));
        assert_eq!(add_two_numbers(from(2), from(999)), from(1001));
    }
}
