use std::result::Result;
use std::num::ParseFloatError;
use std::iter::Peekable;
use std::str::Chars;
use std::option::Option;

pub fn evaluate(line: &str) -> Result<f32, ParseFloatError> {
    let mut iter = line.chars().peekable();
    Ok(try!(parse_expression(&mut iter.by_ref())))
}

pub fn parse_expression(iter: &mut Peekable<Chars>) -> Result<f32, ParseFloatError> {
    let mut accumulator = try!(parse_term(&mut iter.by_ref()));
    loop {
        accumulator = match next_symbol(&mut iter.by_ref()) {
            Some('+') => { try!(do_addition(accumulator, &mut iter.by_ref())) },
            Some('-') => { try!(do_subtitution(accumulator, &mut iter.by_ref())) },
            Some(_) | None => break,
        }
    }
    Ok(accumulator)
}

pub fn parse_term(iter: &mut Peekable<Chars>) -> Result<f32, ParseFloatError> {
    let mut accumulator = try!(parse_arg(&mut iter.by_ref()));
    loop {
        accumulator = match next_symbol(&mut iter.by_ref()) {
            Some('×') => { try!(do_multiplication(accumulator, &mut iter.by_ref())) },
            Some('÷') => { try!(do_division(accumulator, &mut iter.by_ref())) },
            Some(_) | None => break,
        }
    }
    Ok(accumulator)
}

fn parse_arg(iter: &mut Peekable<Chars>) -> Result<f32, ParseFloatError> {
    let mut value = vec![];
    loop {
        match next_symbol(&mut iter.by_ref()) {
            Some(c @ '0'...'9') | Some(c @ '.') => { skip_symbol(&mut iter.by_ref()); value.push(c) },
            Some('(') => {
                skip_symbol(&mut iter.by_ref());
                let result = try!(parse_expression(&mut iter.by_ref()));
                skip_symbol(&mut iter.by_ref());
                return Ok(result);
            },
            Some(_) | None => break,
        }
    }
    value.iter().cloned().collect::<String>().parse::<f32>()
}

fn next_symbol(iter: &mut Peekable<Chars>) -> Option<char> {
    iter.peek().cloned()
}

fn skip_symbol(iter: &mut Peekable<Chars>) {
    iter.next();
}

fn do_addition(value: f32, iter: &mut Peekable<Chars>) -> Result<f32, ParseFloatError> {
    skip_symbol(&mut iter.by_ref());
    Ok(value + try!(parse_term(&mut iter.by_ref())))
}

fn do_subtitution(value: f32, iter: &mut Peekable<Chars>) -> Result<f32, ParseFloatError> {
    skip_symbol(&mut iter.by_ref());
    Ok(value - try!(parse_term(&mut iter.by_ref())))
}

fn do_multiplication(value: f32, iter: &mut Peekable<Chars>) -> Result<f32, ParseFloatError> {
    skip_symbol(&mut iter.by_ref());
    Ok(value * try!(parse_arg(&mut iter.by_ref())))
}

fn do_division(value: f32, iter: &mut Peekable<Chars>) -> Result<f32, ParseFloatError> {
    skip_symbol(&mut iter.by_ref());
    Ok(value / try!(parse_arg(&mut iter.by_ref())))
}

#[test]
fn test_eval_simple_num() {
    assert_eq!(evaluate("1"), Ok(1.0));
}

#[test]
fn test_eval_three_digit_num() {
    assert_eq!(evaluate("123"), Ok(123.0));
}

#[test]
fn test_eval_real_num() {
    assert_eq!(evaluate("12.035"), Ok(12.035));
}

#[test]
fn test_eval_add() {
    assert_eq!(evaluate("1+2"), Ok(3.0));
}

#[test]
fn test_eval_sub() {
    assert_eq!(evaluate("3-1"), Ok(2.0));
}

#[test]
fn test_eval_mul() {
    assert_eq!(evaluate("3×3"), Ok(9.0));
}

#[test]
fn test_eval_div() {
    assert_eq!(evaluate("10÷2"), Ok(5.0));
}

#[test]
fn test_eval_few_operations() {
    assert_eq!(evaluate("2+3-1+10"), Ok(14.0));
}

#[test]
fn test_eval_operations_with_diff_priority() {
    assert_eq!(evaluate("2+3×9-20÷5+1"), Ok(26.0));
}

#[test]
fn test_eval_operation_with_parentheses() {
    assert_eq!(evaluate("2+(7-4)×3-10"), Ok(1.0));
}
