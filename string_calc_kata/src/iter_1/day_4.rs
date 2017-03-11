use std::iter::Peekable;
use std::str::Chars;

pub fn evaluate(line: String) -> f32 {
    let mut iter = line.chars().peekable();
    let mut accumulator = parse_arg(&mut iter);
    while iter.peek().is_some() {
        let sign = iter.peek().cloned();
        match sign {
            Some('+') => { iter.next(); accumulator += evaluate(iter.by_ref().collect()) },
            Some('-') => { iter.next(); accumulator -= evaluate(iter.by_ref().collect()) },
            Some('×') => { iter.next(); accumulator *= parse_arg(&mut iter) },
            Some('÷') => { iter.next(); accumulator /= parse_arg(&mut iter) },
            _ => break,
        }
    }
    accumulator
}

fn parse_arg(iter: &mut Peekable<Chars>) -> f32 {
    let mut accumulator = 0.0;
    let mut exponent = 0.1;
    let mut has_point = false;
    while iter.peek().is_some() && is_number_symbol(iter.peek().unwrap()) {
        let c = iter.next().unwrap();
        if c == '.' {
            has_point = true;
            continue;
        }
        let value = c.to_digit(10).unwrap() as f32;
        if has_point {
            accumulator += value * exponent;
            exponent *= 0.1;
        }
        else {
            accumulator = accumulator*10.0 + value;
        }
    }
    accumulator
}

fn is_number_symbol(c: &char) -> bool {
    c.is_digit(10) || *c == '.'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_one() {
        assert_eq!(evaluate("1".to_string()), 1.0);
    }

    #[test]
    fn test_evaluate_one_hundred() {
        assert_eq!(evaluate("100".to_string()), 100.0);
    }

    #[test]
    fn test_evaluate_add() {
        assert_eq!(evaluate("5+4".to_string()), 9.0);
    }

    #[test]
    fn test_evaluate_sub() {
        assert_eq!(evaluate("5-4".to_string()), 1.0);
    }

    #[test]
    fn test_evaluate_mul() {
        assert_eq!(evaluate("5×4".to_string()), 20.0);
    }

    #[test]
    fn test_evaluate_div() {
        assert_eq!(evaluate("20÷5".to_string()), 4.0);
    }

    #[test]
    fn test_evaluate_float_number() {
        assert_eq!(evaluate("100.254".to_string()), 100.254)
    }

    #[test]
    fn test_evaluate_two_add() {
        assert_eq!(evaluate("10+11+5".to_string()), 26.0);
    }

    #[test]
    fn test_evaluate_operation_with_different_priority() {
        assert_eq!(evaluate("2+2×2".to_string()), 6.0);
    }
}