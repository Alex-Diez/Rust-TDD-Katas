use std::borrow::Cow;
use std::num::ParseFloatError;
use std::str::{FromStr, Chars};
use std::iter::Peekable;

pub fn evaluate<'s>(src: Cow<'s, str>) -> Result<f64, ParseFloatError> {
    let mut chars = (*src).chars().peekable();
    parse_expression(chars.by_ref())
}

fn parse_expression(chars: &mut Peekable<Chars>) -> Result<f64, ParseFloatError> {
    let mut result = parse_term(chars.by_ref())?;
    loop {
        match chars.peek().cloned() {
            Some('+') => {
                chars.next();
                result += parse_term(chars.by_ref())?;
            }
            Some('-') => {
                chars.next();
                result -= parse_term(chars.by_ref())?;
            }
            _ => break
        }
    }
    Ok(result)
}

fn parse_term(chars: &mut Peekable<Chars>) -> Result<f64, ParseFloatError> {
    let mut result = parse_arg(chars.by_ref())?;
    loop {
        match chars.peek().cloned() {
            Some('×') => {
                chars.next();
                result *= parse_arg(chars.by_ref())?;
            }
            Some('÷') => {
                chars.next();
                result /= parse_arg(chars.by_ref())?;
            }
            _ => break
        }
    }
    Ok(result)
}

fn parse_arg(chars: &mut Peekable<Chars>) -> Result<f64, ParseFloatError> {
    let mut buffer = String::new();
    loop {
        match chars.peek().cloned() {
            Some(d @ '0' ... '9') => buffer.push(d),
            Some('(') => {
                chars.next();
                let ret = parse_expression(chars.by_ref());
                if let Some(')') = chars.peek().cloned() {
                    chars.next();
                }
                return ret;
            }
            _ => break
        }
        chars.next();
    }
    f64::from_str(buffer.as_ref())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::Cow;

    #[test]
    fn evaluate_number() {
        assert_eq!(evaluate(Cow::Borrowed("1.0")), Ok(1.0));
    }

    #[test]
    fn evaluate_add() {
        assert_eq!(evaluate(Cow::Borrowed("1+3")), Ok(4.0));
    }

    #[test]
    fn evaluate_sub() {
        assert_eq!(evaluate(Cow::Borrowed("4-5")), Ok(-1.0));
    }

    #[test]
    fn evaluate_mul() {
        assert_eq!(evaluate(Cow::Borrowed("3×5")), Ok(15.0));
    }

    #[test]
    fn evaluate_div() {
        assert_eq!(evaluate(Cow::Borrowed("35÷7")), Ok(5.0));
    }

    #[test]
    fn evaluate_many_operations() {
        assert_eq!(evaluate(Cow::Borrowed("5-3+12÷4-1×5+12")), Ok(12.0));
    }

    #[test]
    fn evaluate_expression_with_parenthesis() {
        assert_eq!(evaluate(Cow::Borrowed("5-((3+12)÷(4-1))×(5+12)")), Ok(-80.0));
    }
}
