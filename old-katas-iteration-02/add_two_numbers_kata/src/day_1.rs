use std::fmt;

pub fn add_two_numbers(first: ListNode, second: ListNode) -> ListNode {
    match (first, second) {
        (Some(first), Some(second)) => {
            let val = (*first).val + (*second).val;
            if val > 9 {
                if let Some(next) = (*first).next {
                    List::with_next(val % 10, add_two_numbers(Some(next), List::new(val / 10)))
                } else {
                    List::with_next(val % 10, add_two_numbers(List::new(val / 10), (*second).next))
                }
            } else {
                List::with_next(val, add_two_numbers((*first).next, (*second).next))
            }
        },
        (Some(first), None) => Some(first),
        (None, Some(second)) => Some(second),
        _ => None
    }
}

type ListNode = Option<Box<List>>;

#[derive(PartialEq)]
pub struct List {
    val: u32,
    next: ListNode,
}

impl List {
    pub fn new(val: u32) -> ListNode {
        Some(Box::new(List { val, next: None }))
    }

    pub fn with_next(val: u32, next: ListNode) -> ListNode {
        Some(Box::new(List { val, next }))
    }
}

impl ToString for List {
    fn to_string(&self) -> String {
        match self.next.as_ref() {
            None => self.val.to_string(),
            Some(next) => self.val.to_string() + "->" + next.to_string().as_str()
        }
    }
}

impl fmt::Debug for List {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn from_num(num: u32) -> ListNode {
        if num > 9 {
            List::with_next(num % 10, from_num(num / 10))
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
        assert_eq!(add_two_numbers(List::new(1), List::new(1)), List::new(2));
    }

    #[test]
    fn add_single_digit_numbers_with_overflow() {
        assert_eq!(add_two_numbers(List::new(5), List::new(6)), from_num(11));
    }

    #[test]
    fn add_two_digits_numbers_without_overflow() {
        assert_eq!(add_two_numbers(from_num(12), from_num(23)), from_num(35));
    }

    #[test]
    fn add_two_digits_numbers_with_overflow() {
        assert_eq!(add_two_numbers(from_num(90), from_num(11)), from_num(101));
        assert_eq!(add_two_numbers(from_num(12), from_num(90)), from_num(102));
    }

    #[test]
    fn add_two_digits_numbers_with_overflow_from_first_digit() {
        assert_eq!(add_two_numbers(from_num(99), from_num(2)), from_num(101));
        assert_eq!(add_two_numbers(from_num(3), from_num(99)), from_num(102));
    }

    #[test]
    fn add_three_digits_numbers_without_overflow() {
        assert_eq!(add_two_numbers(from_num(111), from_num(222)), from_num(333));
    }

    #[test]
    fn add_three_digits_numbers_with_overflow() {
        assert_eq!(add_two_numbers(from_num(900), from_num(301)), from_num(1201));
        assert_eq!(add_two_numbers(from_num(302), from_num(900)), from_num(1202));

        assert_eq!(add_two_numbers(from_num(990), from_num(13)), from_num(1003));
        assert_eq!(add_two_numbers(from_num(24), from_num(980)), from_num(1004));

        assert_eq!(add_two_numbers(from_num(999), from_num(3)), from_num(1002));
        assert_eq!(add_two_numbers(from_num(4), from_num(999)), from_num(1003));
    }
}
