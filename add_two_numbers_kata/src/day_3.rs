type ListNode = Option<Box<List>>;

pub fn add_two_numbers(first: ListNode, second: ListNode) -> ListNode {
    match (first, second) {
        (Some(first), Some(second)) => {
            let first = *first;
            let second = *second;
            let val = first.val + second.val;
            if val > 9 {
                List::with_next(val % 10, add_two_numbers(add_two_numbers(first.next, List::new(val / 10)), second.next))
            } else {
                List::with_next(val, add_two_numbers(first.next, second.next))
            }
        },
        (Some(first), None) => Some(first),
        (None, Some(second)) => Some(second),
        _ => None
    }
}

#[derive(PartialEq, Debug)]
pub struct List {
    val: u32,
    next: ListNode
}

impl List {
    pub fn new(val: u32) -> ListNode {
        Some(Box::new(List { val, next: None }))
    }

    pub fn with_next(val: u32, next: ListNode) -> ListNode {
        Some(Box::new(List { val, next }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn from(num: u32) -> ListNode {
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
    fn add_single_digit_numbers() {
        assert_eq!(add_two_numbers(List::new(4), List::new(5)), List::new(9));
    }

    #[test]
    fn add_single_digit_numbers_with_overflow() {
        assert_eq!(add_two_numbers(List::new(7), List::new(8)), from(15));
    }
    
    #[test]
    fn add_two_digits_numbers() {
        assert_eq!(add_two_numbers(from(11), from(22)), from(33));
    }

    #[test]
    fn add_two_digits_numbers_with_overflow() {
        assert_eq!(add_two_numbers(from(11), from(99)), from(110));
    }

    #[test]
    fn add_three_digits_numbers() {
        assert_eq!(add_two_numbers(from(111), from(222)), from(333));
    }

    #[test]
    fn add_three_digits_numbers_with_overflow() {
        assert_eq!(add_two_numbers(from(900), from(100)), from(1000));
        assert_eq!(add_two_numbers(from(990), from(10)), from(1000));
        assert_eq!(add_two_numbers(from(999), from(1)), from(1000));

        assert_eq!(add_two_numbers(from(100), from(900)), from(1000));
        assert_eq!(add_two_numbers(from(10), from(990)), from(1000));
        assert_eq!(add_two_numbers(from(1), from(999)), from(1000));
    }
}
