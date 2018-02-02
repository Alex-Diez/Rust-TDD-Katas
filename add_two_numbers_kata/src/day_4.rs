type ListNode = Option<Box<List>>;

pub fn add_two_numbers(first: ListNode, second: ListNode) -> ListNode {
    add_two_numbers_with_overflow(first, second, None)
}

fn add_two_numbers_with_overflow(first: ListNode, second: ListNode, overflow: ListNode) -> ListNode {
    match (first, second, overflow) {
        (Some(first), None, Some(overflow)) =>
            add_two_numbers_with_overflow(Some(first), Some(overflow), None),
        (None, Some(second), Some(overflow)) =>
            add_two_numbers_with_overflow(Some(second), Some(overflow), None),
        (Some(first), Some(second), overflow) => {
            let mut num = (*first).val + (*second).val;
            if let Some(overflow) = overflow {
               num += (*overflow).val;
            }
            List::with_next(
                num % 10,
                add_two_numbers_with_overflow(first.next, second.next, List::new(num / 10))
            )
        }
        (_, _, Some(overflow)) => {
            if overflow.val == 0 {
                None
            } else {
                Some(overflow)
            }
        }
        _ => None
    }
}

#[derive(PartialEq, Debug)]
pub struct List {
    val: u32,
    next: ListNode,
}

impl List {
    pub fn new(val: u32) -> ListNode {
        List::with_next(val, None)
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
    fn add_single_digit_numbers_without_overflow() {
        assert_eq!(add_two_numbers(List::new(1), List::new(2)), List::new(3));
    }

    #[test]
    fn add_single_digit_numbers_with_overflow() {
        assert_eq!(add_two_numbers(List::new(8), List::new(9)), from(17));
    }

    #[test]
    fn add_two_digits_numbers_without_overflow() {
        assert_eq!(add_two_numbers(from(11), from(22)), from(33));
    }

    #[test]
    fn add_two_digits_numbers_with_overflow() {
        assert_eq!(add_two_numbers(from(90), from(10)), from(100));

        assert_eq!(add_two_numbers(from(99), from(2)), from(101));
        assert_eq!(add_two_numbers(from(2), from(99)), from(101));
    }

    #[test]
    fn add_three_digits_numbers_with_overflow() {
        assert_eq!(add_two_numbers(from(901), from(101)), from(1002));
        assert_eq!(add_two_numbers(from(991), from(11)), from(1002));
        assert_eq!(add_two_numbers(from(999), from(3)), from(1002));
    }
}
