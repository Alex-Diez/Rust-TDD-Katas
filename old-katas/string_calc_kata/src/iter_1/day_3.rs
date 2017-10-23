use std::iter::Peekable;
use std::str::Chars;
use std::option::Option;

pub fn evaluate(line: &str) -> f32 {
    let mut chars = line.chars().peekable();
    let mut accumulator = parse_term(chars.by_ref());
    while chars.peek().is_some() {
        let sign = parse_sign(chars.by_ref());
        accumulator = match sign {
            Some('+') => { chars.next(); accumulator + parse_term(chars.by_ref()) }
            Some('-') => { chars.next(); accumulator - parse_term(chars.by_ref()) }
            _ => break
        }
    }
    accumulator
}

fn parse_term(chars: &mut Peekable<Chars>) -> f32 {
    let mut accumulator = parse_arg(chars.by_ref());
    while chars.peek().is_some() {
        let sign = parse_sign(chars.by_ref());
        accumulator = match sign {
            Some('x') => { chars.next(); accumulator * parse_arg(chars.by_ref()) }
            Some('÷') => { chars.next(); accumulator / parse_arg(chars.by_ref()) }
            _ => break
        }
    }
    accumulator
}

fn parse_arg(chars: &mut Peekable<Chars>) -> f32 {
    let mut accumulator = 0.0;
    let mut point = false;
    let mut exponent = 0.1;
    while chars.peek().is_some() && is_number_symbol(chars.peek().unwrap()) {
        let v = chars.next().unwrap();
        if v == '.' {
            point = true;
            continue;
        }
        let digit = v.to_digit(10).unwrap() as f32;
        if point {
            accumulator += digit * exponent;
            exponent *= 0.1;
        } else {
            accumulator = accumulator * 10.0 + digit;
        }
    }
    accumulator
}

fn parse_sign(chars: &mut Peekable<Chars>) -> Option<char> {
    chars.peek().cloned()
}

fn is_number_symbol(c: &char) -> bool {
    c.is_digit(10) || *c == '.'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_simple_number() {
        assert_eq!(evaluate("1"), 1.0);
    }

    #[test]
    fn test_evaluate_long_number() {
        assert_eq!(evaluate("100"), 100.0);
    }

    #[test]
    fn test_evaluate_one_plus_two() {
        assert_eq!(evaluate("1+2"), 3.0);
    }

    #[test]
    fn test_evaluate_two_minus_one() {
        assert_eq!(evaluate("2-1"), 1.0);
    }

    #[test]
    fn test_evaluate_multiplication() {
        assert_eq!(evaluate("2x4"), 8.0);
    }

    #[test]
    fn test_evaluate_division() {
        assert_eq!(evaluate("4÷2"), 2.0);
    }

    #[test]
    fn test_evaluate_real_number() {
        assert_eq!(evaluate("1.012"), 1.012);
    }

    #[test]
    fn test_evaluate_three_plus_four_plus_two() {
        assert_eq!(evaluate("3+4+2"), 9.0);
    }

    #[test]
    fn test_evaluate_operation_with_diff_priority() {
        assert_eq!(evaluate("3+4x2"), 11.0);
    }
}
