use std::iter::Peekable;
use std::str::Chars;

pub fn evaluate(line: &str) -> f32 {
    evaluate_iter(&mut line.chars().peekable())
}

fn evaluate_iter(iter: &mut Peekable<Chars>) -> f32 {
    let mut accumulator = parse_term(iter.by_ref());
    while iter.peek().is_some() {
        let sign = iter.peek().cloned();
        match sign {
            Some('+') => { iter.next(); accumulator += parse_term(iter.by_ref()) },
            Some('-') => { iter.next(); accumulator -= parse_term(iter.by_ref()) },
            Some(_) | None => break,
        }
    }
    accumulator
}

fn parse_term(iter: &mut Peekable<Chars>) -> f32 {
    let mut accumulator = parse_arg(iter.by_ref());
    while iter.peek().is_some() {
        let sign = iter.peek().cloned();
        match sign {
            Some('×') => { iter.next(); accumulator *= parse_arg(iter.by_ref()) },
            Some('÷') => { iter.next(); accumulator /= parse_arg(iter.by_ref()) },
            Some(_) | None => break,
        }
    }
    accumulator
}

fn parse_arg(iter: &mut Peekable<Chars>) -> f32 {
    let mut has_point = false;
    let mut accumulator = 0.0;
    let mut exponent = 0.1;
    while iter.peek().is_some() && (iter.peek().unwrap().is_digit(10) || *iter.peek().unwrap() == '.') {
        let symbol = iter.next();
        match symbol {
            Some('.') => {
                has_point = true;
                continue
            },
            Some(d @ '0'...'9') => {
                let v = d.to_digit(10).unwrap() as f32;
                if has_point {
                    accumulator += v * exponent;
                    exponent *= 0.1;
                }
                else {
                    accumulator = accumulator*10.0 + v;
                }
            },
            _ => break,
        }
    }
    accumulator
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_simple_number() {
        assert_eq!(evaluate("1"), 1.0);
    }

    #[test]
    fn test_evaluate_big_number() {
        assert_eq!(evaluate("100"), 100.0);
    }

    #[test]
    fn test_evaluate_real_number() {
        assert_eq!(evaluate("1.09"), 1.09)
    }

    #[test]
    fn test_evaluate_add() {
        assert_eq!(evaluate("1.09+1.01"), 2.1);
    }

    #[test]
    fn test_evaluate_sub() {
        assert_eq!(evaluate("2-1"), 1.0);
    }

    #[test]
    fn test_evaluate_mul() {
        assert_eq!(evaluate("2×2"), 4.0);
    }

    #[test]
    fn test_evaluate_div() {
        assert_eq!(evaluate("22÷2"), 11.0);
    }

    #[test]
    fn test_two_adds() {
        assert_eq!(evaluate("2+3+6"), 11.0);
    }

    #[test]
    fn test_two_subs() {
        assert_eq!(evaluate("6-4-1"), 1.0);
    }

    #[test]
    fn test_operation_with_different_priority() {
        assert_eq!(evaluate("2+3×2"), 8.0);
    }
}
