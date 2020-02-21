type ListNode = Option<Box<List>>;

pub fn add_two_numbers(first: ListNode, second: ListNode) -> ListNode {
    match (first, second) {
        (Some(first), Some(second)) => {
            let num = (*first).val + (*second).val;
            if num > 9 {
                List::with_next(num % 10, add_two_numbers(add_two_numbers((*first).next, List::from(num / 10)), (*second).next))
            } else {
                List::with_next(num, add_two_numbers((*first).next, (*second).next))
            }
        }
        (Some(first), None) => Some(first),
        (None, Some(second)) => Some(second),
        (None, None) => None
    }
}

#[derive(PartialEq, Debug)]
pub struct List {
    val: u32,
    next: ListNode,
}

impl List {
    fn with_next(val: u32, next: ListNode) -> ListNode {
        Some(Box::new(List { val, next }))
    }

    pub fn from(num: u32) -> ListNode {
        if num > 9 {
            List::with_next(num % 10, List::from(num / 10))
        } else {
            List::with_next(num, None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_zeros() {
        assert_eq!(add_two_numbers(List::from(0), List::from(0)), List::from(0));
    }

    #[test]
    fn add_single_digit_numbers_without_overflow() {
        assert_eq!(add_two_numbers(List::from(1), List::from(2)), List::from(3));
    }

    #[test]
    fn add_single_digit_numbers_with_overflow() {
        assert_eq!(add_two_numbers(List::from(8), List::from(9)), List::from(17));
    }

    #[test]
    fn add_two_digits_numbers_without_overflow() {
        assert_eq!(add_two_numbers(List::from(11), List::from(22)), List::from(33));
    }

    #[test]
    fn add_two_digits_numbers_with_overflow() {
        assert_eq!(add_two_numbers(List::from(10), List::from(90)), List::from(100));
    }

    #[test]
    fn add_two_digits_and_single_digit_numbers_with_overflow() {
        assert_eq!(add_two_numbers(List::from(99), List::from(1)), List::from(100));
        assert_eq!(add_two_numbers(List::from(1), List::from(99)), List::from(100));
    }
}
