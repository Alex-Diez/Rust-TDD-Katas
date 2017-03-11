use std::result::Result;
use std::num::ParseFloatError;
use std::str::Chars;
use std::iter::Peekable;

pub fn evaluate(line: &str) -> Result<f64, ParseFloatError> {
    let mut iter = line.chars().peekable();
    let mut accumulator = try!(parse_term(iter.by_ref()));
    loop {
        match iter.peek() {
            Some(&'+') => { iter.next(); accumulator += try!(parse_term(iter.by_ref())) },
            Some(&'-') => { iter.next(); accumulator -= try!(parse_term(iter.by_ref())) },
            Some(_) | None => break,
        }
    }
    Ok(accumulator)
}

fn parse_term(iter: &mut Peekable<Chars>) -> Result<f64, ParseFloatError> {
    let mut accumulator = try!(parse_factor(iter.by_ref()));
    loop {
        match iter.peek() {
            Some(&'×') => { iter.next(); accumulator *= try!(parse_factor(iter.by_ref())) },
            Some(&'÷') => { iter.next(); accumulator /= try!(parse_factor(iter.by_ref())) },
            Some(_) | None => break,
        }
    }
    Ok(accumulator)
}

fn parse_factor(iter: &mut Peekable<Chars>) -> Result<f64, ParseFloatError> {
    let mut s = vec![];
    loop {
        match iter.peek().cloned() {
            Some(v @ '0'...'9') | Some(v @ '.') => { iter.next(); s.push(v) },
            Some('(') => { iter.next(); return evaluate(&iter.take_while(|c| c != &')').collect::<String>()) },
            Some(_) | None => break,
        }
    }
    s.iter().cloned().collect::<String>().parse::<f64>()
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_eval_simple_num() {
        assert_eq!(evaluate("1"), Ok(1.0));
    }


    #[test]
    fn test_eval_three_digit_num() {
        assert_eq!(evaluate("100"), Ok(100.0));
    }

    #[test]
    fn test_eval_real_num() {
        assert_eq!(evaluate("10.025"), Ok(10.025));
    }

    #[test]
    fn test_eval_add() {
        assert_eq!(evaluate("1+2"), Ok(3.0));
    }

    #[test]
    fn test_eval_sub() {
        assert_eq!(evaluate("2-1"), Ok(1.0));
    }

    #[test]
    fn test_eval_mul() {
        assert_eq!(evaluate("2×3"), Ok(6.0));
    }

    #[test]
    fn test_eval_div() {
        assert_eq!(evaluate("6÷2"), Ok(3.0));
    }

    #[test]
    fn test_eval_two_operation() {
        assert_eq!(evaluate("3+2-1"), Ok(4.0));
    }

    #[test]
    fn test_eval_operation_with_diff_priority() {
        assert_eq!(evaluate("2+2×2-2+6÷3"), Ok(6.0));
    }

    #[test]
    fn test_eval_with_parentheses() {
        assert_eq!(evaluate("2+(3-1)×6-2"), Ok(12.0));
    }
}