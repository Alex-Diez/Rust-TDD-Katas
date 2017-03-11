use std::result::Result;
use std::num::ParseFloatError;
use std::str::Chars;
use std::iter::Peekable;

pub fn evaluate(line: &str) -> Result<f64, ParseFloatError> {
    let mut iter = line.chars().peekable();
    Ok(try!(parse_expression(&mut iter)))
}

fn parse_expression(iter: &mut Peekable<Chars>) -> Result<f64, ParseFloatError> {
    let mut value = try!(parse_term(&mut iter.by_ref()));
    loop {
        value = match next_symbol(&mut iter.by_ref()) {
            Some('+') => try!(do_addition(value, &mut iter.by_ref())),
            Some('-') => try!(do_substitution(value, &mut iter.by_ref())),
            Some(_) | None => break,
        }
    }
    Ok(value)
}

fn parse_term(iter: &mut Peekable<Chars>) -> Result<f64, ParseFloatError> {
    let mut value = try!(parse_arg(&mut iter.by_ref()));
    loop {
        value = match next_symbol(&mut iter.by_ref()) {
            Some('×') => try!(do_multiplicatio(value, &mut iter.by_ref())),
            Some('÷') => try!(do_division(value, &mut iter.by_ref())),
            Some(_) | None => break,
        }
    }
    Ok(value)
}
    
fn parse_arg(iter: &mut Peekable<Chars>) -> Result<f64, ParseFloatError> {
    let mut value = vec![];
    loop {
        match next_symbol(&mut iter.by_ref()) {
            Some(c @ '0'...'9') | Some(c @ '.') => { skip_symbol(&mut iter.by_ref()); value.push(c) },
            Some('(') => { skip_symbol(&mut iter.by_ref()); let result = try!(parse_expression(&mut iter.by_ref())); skip_symbol(&mut iter.by_ref()); return Ok(result); },
            Some(_) | None => break,
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

fn do_multiplicatio(value: f64, iter: &mut Peekable<Chars>) -> Result<f64, ParseFloatError> {
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
    fn test_eval_simle_num() {
        assert_eq!(evaluate("1"), Ok(1.0));
    }

    #[test]
    fn test_eval_three_digit_num() {
        assert_eq!(evaluate("123"), Ok(123.0));
    }

    #[test]
    fn test_eval_real_num() {
        assert_eq!(evaluate("12.256"), Ok(12.256));
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
    fn test_eval_few_operation() {
        assert_eq!(evaluate("2+3-1"), Ok(4.0));
    }

    #[test]
    fn test_eval_mul() {
        assert_eq!(evaluate("2×3"), Ok(6.0));
    }

    #[test]
    fn test_eval_div() {
        assert_eq!(evaluate("10÷2"), Ok(5.0));
    }

    #[test]
    fn test_eval_operation_with_diffrenet_priorities() {
        assert_eq!(evaluate("1+2×5-6÷2-4"), Ok(4.0));
    }

    #[test]
    fn test_eval_with_parentheses() {
        assert_eq!(evaluate("1+(5-3)×2-(6+1)"), Ok(-2.0));
    }

    #[test]
    fn test_eval_with_two_level_of_parentheses() {
        assert_eq!(evaluate("1+(5-(2+1)+4)×3-(4+2)"), Ok(13.0));
    }
}
