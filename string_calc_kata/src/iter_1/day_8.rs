use std::result::Result;
use std::num::ParseFloatError;
use std::str::Chars;
use std::iter::Peekable;

pub fn evaluate(line: &str) -> Result<f32, ParseFloatError> {
    let mut iter = line.chars().peekable();
    Ok(try!(parse_expression(&mut iter)))
}

fn parse_expression(iter: &mut Peekable<Chars>) -> Result<f32, ParseFloatError> {
    let mut accumulator = try!(parse_term(&mut iter.by_ref()));
    loop {
        let sign = iter.peek().cloned();
        match sign {
            Some('+') => { iter.next(); accumulator += try!(parse_term(&mut iter.by_ref())) },
            Some('-') => { iter.next(); accumulator -= try!(parse_term(&mut iter.by_ref())) },
            Some(_) | None => break,
        }
    }
    Ok(accumulator)
}

fn parse_term(iter: &mut Peekable<Chars>) -> Result<f32, ParseFloatError> {
    let mut accumulator = try!(parse_factor(iter.by_ref()));
    loop {
        let sign = iter.peek().cloned();
        match sign {
            Some('×') => { iter.next(); accumulator *= try!(parse_factor(&mut iter.by_ref())) },
            Some('÷') => { iter.next(); accumulator /= try!(parse_factor(&mut iter.by_ref())) },
            Some(_) | None => break,
        }
    }
    Ok(accumulator)
}

fn parse_factor(iter: &mut Peekable<Chars>) -> Result<f32, ParseFloatError> {
    let mut value = vec![];
    loop {
        let symbol = iter.peek().cloned();
        match symbol {
            Some(c @ '0'...'9') | Some(c @ '.') => { iter.next(); value.push(c) },
            Some(_) | None => break,
        }
    }
    value.iter().cloned().collect::<String>().parse::<f32>()
}

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
    assert_eq!(evaluate("10.25"), Ok(10.25));
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
fn test_eval_fiew_operation() {
    assert_eq!(evaluate("6+8-10"), Ok(4.0));
}

#[test]
fn test_eval_operation_with_diff_priority() {
    assert_eq!(evaluate("6+8×3-10÷2"), Ok(25.0))
}
