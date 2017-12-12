use std::borrow::Cow;
use std::iter::Peekable;
use std::num::ParseFloatError;
use std::str::Chars;

pub fn calculate<'s>(src: Cow<'s, str>) -> Result<f64, ParseFloatError> {
    let mut iter = src.chars().peekable();
    parse_expression(iter.by_ref())
}

fn parse_expression(iter: &mut Peekable<Chars>) -> Result<f64, ParseFloatError> {
    let mut ret = parse_term(iter.by_ref());
    loop {
        match iter.peek().cloned() {
            Some('+') => {
                iter.next();
                ret = ret.and_then(|ret| parse_term(iter.by_ref()).map(|item| ret + item))
            },
            Some('-') => {
                iter.next();
                ret = ret.and_then(|ret| parse_term(iter.by_ref()).map(|item| ret - item))
            }
            _ => break
        }
    }
    ret
}

fn parse_term(iter: &mut Peekable<Chars>) -> Result<f64, ParseFloatError> {
    let mut ret = parse_num(iter.by_ref());
    loop {
        match iter.peek().cloned() {
            Some('×') => {
                iter.next();
                ret = ret.and_then(|ret| parse_num(iter.by_ref()).map(|item| ret * item))
            },
            Some('÷') => {
                iter.next();
                ret = ret.and_then(|ret| parse_num(iter.by_ref()).map(|item| ret / item))
            }
            _ => break
        }
    }
    ret
}

fn parse_num(iter: &mut Peekable<Chars>) -> Result<f64, ParseFloatError> {
    let mut num = String::new();
    loop {
        match iter.peek().cloned() {
            Some('+') | Some('×') | Some('÷') | Some(')') | None => break,
            Some('-') if !num.is_empty() => break,
            Some('(') => {
                iter.next();
                let ret = parse_expression(iter.by_ref());
                iter.next();
                return ret;
            }
            Some(d) => num.push(d)
        }
        iter.next();
    }
    num.parse()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluate_negative_number() {
        assert_eq!(calculate(Cow::Borrowed("-65")), Ok(-65.0));
    }

    #[test]
    fn evaluate_addition() {
        assert_eq!(calculate(Cow::Borrowed("34+12")), Ok(46.0));
    }

    #[test]
    fn evaluate_subtraction() {
        assert_eq!(calculate(Cow::Borrowed("21-45")), Ok(-24.0));
    }

    #[test]
    fn evaluate_multiplication() {
        assert_eq!(calculate(Cow::Borrowed("32×2")), Ok(64.0));
    }

    #[test]
    fn evaluate_division() {
        assert_eq!(calculate(Cow::Borrowed("42÷7")), Ok(6.0));
    }

    #[test]
    fn evaluate_many_operations() {
        assert_eq!(calculate(Cow::Borrowed("2×21-45÷9+1")), Ok(38.0));
    }

    #[test]
    fn evaluate_operations_with_parenthesis() {
        assert_eq!(calculate(Cow::Borrowed("2×((21-48)÷(8+1))")), Ok(-6.0));
    }
}
