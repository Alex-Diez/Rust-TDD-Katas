use std::iter::Peekable;
use std::str::Chars;

pub fn evaluate(line: &str) -> f32 {
    parse_expre(&mut line.chars().peekable())
}

fn parse_expre(iter: &mut Peekable<Chars>) -> f32 {
    let mut accumulator = parse_term(&mut iter.by_ref());
    println!("10 accumulator - {:?}", accumulator);
    while iter.peek().is_some() {
        match iter.peek() {
            Some(&'+') => { iter.next(); accumulator += parse_term(&mut iter.by_ref()); },
            Some(&'-') => { iter.next(); accumulator -= parse_term(&mut iter.by_ref()); },
            Some(_) | None => break,
        }
    }
    accumulator
}

fn parse_term(iter: &mut Peekable<Chars>) -> f32 {
    let mut accumulator = parse_factor(&mut iter.by_ref());
    println!("23 accumulator - {:?}", accumulator);
    while iter.peek().is_some() {
        match iter.peek() {
            Some(&'×') => { iter.next(); accumulator *= parse_factor(&mut iter.by_ref()) },
            Some(&'÷') => { iter.next(); accumulator /= parse_factor(&mut iter.by_ref()) },
            Some(_) | None => break,
        }
    }
    accumulator
}

fn parse_factor(iter: &mut Peekable<Chars>) -> f32 {
    let mut exp = 0.1;
    let mut accumulator = 0.0;
    let mut has_point = false;
    while iter.peek().is_some() && is_number_symbol(&iter.peek()) {
        match iter.next() {
            Some('.') => has_point = true,
            Some(c @ '0'...'9') => {
                        let value = c.to_digit(10).unwrap() as f32;
                        if has_point {
                            accumulator += value*exp;
                            exp *= 0.1;
                        }
                        else {
                            accumulator = accumulator*10.0 + value;
                        }
                    }
            Some(_) | None => {}
        }
    }
    accumulator
}

fn is_number_symbol(symbol: &Option<&char>) -> bool {
    match *symbol {
        Some(&'.') | Some(&'0'...'9') => true,
        Some(_) | None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_simple_num() {
        assert_eq!(evaluate("1"), 1.0);
    }

    #[test]
    fn test_eval_three_digit_num() {
        assert_eq!(evaluate("100"), 100.0);
    }

    #[test]
    fn test_eval_real_num() {
        assert_eq!(evaluate("1.01"), 1.01)
    }

    #[test]
    fn test_eval_add() {
        assert_eq!(evaluate("1+1"), 2.0);
    }

    #[test]
    fn test_eval_sub() {
        assert_eq!(evaluate("2-1"), 1.0);
    }

    #[test]
    fn test_eval_mul() {
        assert_eq!(evaluate("2×3"), 6.0);
    }

    #[test]
    fn test_eval_div() {
        assert_eq!(evaluate("6÷2"), 3.0);
    }

    #[test]
    fn test_eval_two_operation() {
        assert_eq!(evaluate("3+2-1"), 4.0);
    }

    #[test]
    fn test_eval_operation_with_diff_priority() {
        assert_eq!(evaluate("2+2×2-2+6÷3"), 6.0);
    }
}