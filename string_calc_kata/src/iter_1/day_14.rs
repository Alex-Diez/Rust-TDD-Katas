use std::result::Result;
use std::num::ParseFloatError;
use std::iter::Peekable;
use std::str::Chars;

pub fn evaluate(line: &str) -> Result<f64, ParseFloatError> {
    let mut iter = line.chars().peekable();
    Ok(try!(parse_expression(&mut iter.by_ref())))
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
            Some('×') => try!(do_multiplication(value, &mut iter.by_ref())),
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
    fn test_eval_three_digit_num() {
        assert_eq!(evaluate("256"), Ok(256.0));
    }

    #[test]
    fn test_eval_real_num() {
        assert_eq!(evaluate("125.256"), Ok(125.256));
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
    fn test_eval_few_operations() {
        assert_eq!(evaluate("2+3-1+4"), Ok(8.0));
    }

    #[test]
    fn test_eval_mul() {
        assert_eq!(evaluate("2×5"), Ok(10.0));
    }

    #[test]
    fn test_eval_div() {
        assert_eq!(evaluate("10÷2"), Ok(5.0));
    }

    #[test]
    fn test_eval_operations_with_diff_priority() {
        assert_eq!(evaluate("20+2×5-100÷4"), Ok(5.0));
    }

    #[test]
    fn test_eval_operations_with_parentheses() {
        assert_eq!(evaluate("2+(2-3+5×2)-8"), Ok(3.0));
    }

    #[test]
    fn test_eval_operations_with_two_levels_of_parentheses() {
        assert_eq!(evaluate("2+(2-3+5×2)-((1+1)×4)"), Ok(3.0));
    }
}
