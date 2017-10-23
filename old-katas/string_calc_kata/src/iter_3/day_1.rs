use std::num::ParseFloatError;
use std::iter::Peekable;
use std::str::{Chars, FromStr};

pub fn evaluate(src: &str) -> Result<f64, ParseFloatError> {
    let mut iterator = src.chars().peekable();
    let arg_one = parse_arg(iterator.by_ref())?;
    match iterator.peek().cloned() {
        Some('+') => {
            iterator.next();
            Ok(arg_one + parse_arg(iterator.by_ref())?)
        },
        Some('-') => {
            iterator.next();
            Ok(arg_one - parse_arg(iterator.by_ref())?)
        },
        Some('×') => {
            iterator.next();
            Ok(arg_one * parse_arg(iterator.by_ref())?)
        }
        Some('÷') => {
            iterator.next();
            Ok(arg_one / parse_arg(iterator.by_ref())?)
        }
        Some(_) | None => Ok(arg_one)
    }
}

fn parse_arg(iterator: &mut Peekable<Chars>) -> Result<f64, ParseFloatError> {
    let mut arg = String::new();
    loop {
        match iterator.peek().cloned() {
            Some(c @ '0' ... '9') => {
                iterator.next();
                arg.push(c)
            },
            _ => break
        }
    }
    if arg.is_empty() {
        Ok(0.0)
    } else {
        Ok(f64::from_str(arg.as_ref())?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluate_number() {
        assert_eq!(evaluate("1"), Ok(1.0));
    }

    #[test]
    fn evaluate_add() {
        assert_eq!(evaluate("1+2"), Ok(3.0));
    }

    #[test]
    fn evaluate_sub() {
        assert_eq!(evaluate("2-5"), Ok(-3.0));
    }

    #[test]
    fn evaluate_mul() {
        assert_eq!(evaluate("4×5"), Ok(20.0));
    }

    #[test]
    fn evaluate_div() {
        assert_eq!(evaluate("20÷5"), Ok(4.0));
    }
}
