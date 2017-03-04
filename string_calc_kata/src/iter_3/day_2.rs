use std::num::ParseFloatError;
use std::str::{FromStr, Chars};
use std::borrow::Cow;
use std::iter::Peekable;

pub fn evaluate<'s>(src: Cow<'s, str>) -> Result<f64, ParseFloatError> {
    let owned = src.into_owned();
    let mut chars = owned.chars().peekable();
    let mut term_one = parse_term(chars.by_ref())?;
    loop {
        match chars.peek().cloned() {
            Some('+') => {
                chars.next();
                term_one += parse_term(chars.by_ref())?;
            },
            Some('-') => {
                chars.next();
                term_one -= parse_term(chars.by_ref())?;
            },
            _ => break
        }
    }
    Ok(term_one)
}

fn parse_term(chars: &mut Peekable<Chars>) -> Result<f64, ParseFloatError> {
    let mut arg_one = parse_arg(chars.by_ref())?;
    loop {
        match chars.peek().cloned() {
            Some('×') => {
                chars.next();
                arg_one *= parse_arg(chars.by_ref())?;
            },
            Some('÷') => {
                chars.next();
                arg_one /= parse_arg(chars.by_ref())?;
            },
            _ => break
        }
    }
    Ok(arg_one)
}

fn parse_arg(chars: &mut Peekable<Chars>) -> Result<f64, ParseFloatError> {
    let mut buf = String::new();
    loop {
        match chars.peek().cloned() {
            Some(c @ '0' ... '9') => buf.push(c),
            _ => break
        }
        chars.next();
    }
    f64::from_str(buf.as_ref())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::Cow;

    #[test]
    fn evaluate_number() {
        assert_eq!(evaluate(Cow::Borrowed("1")), Ok(1.0));
    }

    #[test]
    fn evaluate_add() {
        assert_eq!(evaluate(Cow::Borrowed("1+2")), Ok(3.0));
    }

    #[test]
    fn evaluate_sub() {
        assert_eq!(evaluate(Cow::Borrowed("3-1")), Ok(2.0));
    }

    #[test]
    fn evaluate_mul() {
        assert_eq!(evaluate(Cow::Borrowed("12×5")), Ok(60.0));
    }

    #[test]
    fn evaluate_div() {
        assert_eq!(evaluate(Cow::Borrowed("42÷7")), Ok(6.0));
    }

    #[test]
    fn evaluate_multiple_operations() {
        assert_eq!(evaluate(Cow::Borrowed("45-5+10×2-128÷2")), Ok(-4.0));
    }
}
