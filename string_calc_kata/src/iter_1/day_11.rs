use std::result::Result;
use std::num::ParseFloatError;
use std::iter::Peekable;
use std::iter::Iterator;
use std::str::Chars;

pub fn evaluate(line: &str) -> Result<f64, ParseFloatError> {
    let mut iter = line.chars().peekable();
    Ok(try!(parse_expression(&mut iter.by_ref())))
}

fn parse_expression(iter: &mut Peekable<Chars>) -> Result<f64, ParseFloatError> {
    let mut accumulator = try!(parse_term(&mut iter.by_ref()));
    loop {
        accumulator = match next_symbol(&mut iter.by_ref()) {
            Some('+') => try!(do_addition(accumulator, &mut iter.by_ref())),
            Some('-') => try!(do_substitution(accumulator, &mut iter.by_ref())),
            Some(_) | None => break,
        }
    }
    Ok(accumulator)
}

fn parse_term(iter: &mut Peekable<Chars>) -> Result<f64, ParseFloatError> {
    let mut accumulator = try!(parse_arg(&mut iter.by_ref()));
    loop {
        accumulator = match next_symbol(&mut iter.by_ref()) {
            Some('×') => try!(do_multiplication(accumulator, &mut iter.by_ref())),
            Some('÷') => try!(do_division(accumulator, &mut iter.by_ref())),
            Some(_) | None => break,
        }
    }
    Ok(accumulator)
}

fn parse_arg(iter: &mut Peekable<Chars>) -> Result<f64, ParseFloatError> {
    let mut value = vec![];
    loop {
        let symbol = iter.peek().cloned();
        match symbol {
            Some(c @ '0'...'9') | Some(c @ '.') => { iter.next(); value.push(c) },
            Some('(') => { iter.next(); let result = try!(parse_expression(&mut iter.by_ref())); iter.next(); return Ok(result) },
            Some(_) | None => { break }
        }
    }
    value.iter().cloned().collect::<String>().parse::<f64>()
}

fn next_symbol(iter: &mut Peekable<Chars>) -> Option<char> {
    iter.peek().cloned()
}

fn skip_symbol(iter: &mut Peekable<Chars>) {
    iter.next();
}

fn do_addition(value: f64, iter: &mut Peekable<Chars>) -> Result<f64, ParseFloatError> {
    skip_symbol(&mut iter.by_ref());
    Ok(value + try!(parse_term(&mut iter.by_ref())))
}

fn do_substitution(value: f64, iter: &mut Peekable<Chars>) -> Result<f64, ParseFloatError> {
    skip_symbol(&mut iter.by_ref());
    Ok(value - try!(parse_term(&mut iter.by_ref())))
}

fn do_multiplication(value: f64, iter: &mut Peekable<Chars>) -> Result<f64, ParseFloatError> {
    skip_symbol(&mut iter.by_ref());
    Ok(value * try!(parse_arg(&mut iter.by_ref())))
}

fn do_division(value: f64, iter: &mut Peekable<Chars>) -> Result<f64, ParseFloatError> {
    skip_symbol(&mut iter.by_ref());
    Ok(value / try!(parse_arg(&mut iter.by_ref())))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_simple_num() {
        assert_eq!(evaluate("1"), Ok(1.0));
    }

    #[test]
    fn test_eval_big_num() {
        assert_eq!(evaluate("10000"), Ok(10000.0));
    }

    #[test]
    fn test_eval_real_num() {
        assert_eq!(evaluate("10.254"), Ok(10.254));
    }

    #[test]
    fn test_eval_add() {
        assert_eq!(evaluate("10+2"), Ok(12.0));
    }

    #[test]
    fn test_eval_sub() {
        assert_eq!(evaluate("10-2"), Ok(8.0));
    }

    #[test]
    fn test_eval_mul() {
        assert_eq!(evaluate("10×2"), Ok(20.0));
    }

    #[test]
    fn test_eval_div() {
        assert_eq!(evaluate("10÷2"), Ok(5.0));
    }

    #[test]
    fn test_eval_few_op() {
        assert_eq!(evaluate("10+2-1"), Ok(11.0));
    }
}
