use std::result::Result;
use std::num::ParseFloatError;
use std::str::Chars;
use std::iter::Peekable;
use std::option::Option;

pub fn evaluate(line: &str) -> Result<f32, ParseFloatError> {
    Ok(try!(parse_expression(&mut line.chars().peekable())))
}

fn parse_expression(iter: &mut Peekable<Chars>) -> Result<f32, ParseFloatError> {
    let mut value = try!(parse_term(&mut iter.by_ref()));
    loop {
        value = match retrieve_sign(&mut iter.by_ref()) {
            Some('+') => try!(do_addition(value, &mut iter.by_ref())),
            Some('-') => try!(do_subtraction(value, &mut iter.by_ref())),
            Some(_) | None => break,
        }
    }
    Ok(value)
}

fn parse_term(iter: &mut Peekable<Chars>) -> Result<f32, ParseFloatError> {
    let mut value = try!(parse_number(&mut iter.by_ref()));
    loop {
        value = match retrieve_sign(&mut iter.by_ref()) {
            Some('×') => try!(do_multiplation(value, &mut iter.by_ref())),
            Some('÷') => try!(do_division(value, &mut iter.by_ref())),
            Some(_) | None => break,
        }
    }
    Ok(value)
}

fn parse_number(iter: &mut Peekable<Chars>) -> Result<f32, ParseFloatError> {
    let mut value = vec![];
    loop {
        match next_symbol(&mut iter.by_ref()) {
            Some(c @ '0'...'9') | Some(c @ '.') => { skip_symbol(&mut iter.by_ref()); value.push(c) },
            Some('(') => { skip_symbol(&mut iter.by_ref()); let result = Ok(try!(parse_expression(&mut iter.by_ref()))); iter.next(); return result },
            Some(_) | None => break,
        }
    }
    value.iter().cloned().collect::<String>().parse::<f32>()
}

fn retrieve_sign(iter: &mut Peekable<Chars>) -> Option<char> {
    next_symbol(&mut iter.by_ref())
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

fn do_subtraction(value: f32, iter: &mut Peekable<Chars>) -> Result<f32, ParseFloatError> {
    skip_symbol(&mut iter.by_ref());
    Ok(value - try!(parse_term(&mut iter.by_ref())))
}

fn do_multiplation(value: f32, iter: &mut Peekable<Chars>) -> Result<f32, ParseFloatError> {
    skip_symbol(&mut iter.by_ref());
    Ok(value * try!(parse_number(&mut iter.by_ref())))
}

fn do_division(value: f32, iter: &mut Peekable<Chars>) -> Result<f32, ParseFloatError> {
    skip_symbol(&mut iter.by_ref());
    Ok(value / try!(parse_number(&mut iter.by_ref())))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_simple_num() {
        assert_eq!(evaluate("1"), Ok(1.0));
    }

    #[test]
    fn test_three_digit_num() {
        assert_eq!(evaluate("100"), Ok(100.0));
    }

    #[test]
    fn test_eval_real_num() {
        assert_eq!(evaluate("10.25"), Ok(10.25));
    }

    #[test]
    fn test_eval_add() {
        assert_eq!(evaluate("2+1"), Ok(3.0));
    }

    #[test]
    fn test_eval_sub() {
        assert_eq!(evaluate("2-1"), Ok(1.0));
    }

    #[test]
    fn test_eval_mul() {
        assert_eq!(evaluate("2×4"), Ok(8.0));
    }

    #[test]
    fn test_eval_div() {
        assert_eq!(evaluate("6÷2"), Ok(3.0));
    }

    #[test]
    fn test_eval_few_operation() {
        assert_eq!(evaluate("5+8-3+1"), Ok(11.0));
    }

    #[test]
    fn test_eval_operation_with_diff_priority() {
        assert_eq!(evaluate("5+9÷3-2×2+8"), Ok(12.0));
    }

    #[test]
    fn test_eval_operation_with_parentheses() {
        assert_eq!(evaluate("2+(7-5)×3-10"), Ok(-2.0));
    }
}
