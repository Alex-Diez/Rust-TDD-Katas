use std::borrow::Cow;
use std::iter::Peekable;
use std::num::ParseFloatError;
use std::str::Chars;

pub fn calculate<'s>(src: Cow<'s, str>) -> Result<f64, ParseFloatError> {
    let mut iter = src.chars().peekable();
    parse_expression(iter.by_ref())
}

fn parse_expression(iter: &mut Peekable<Chars>) -> Result<f64, ParseFloatError> {
    let mut ret = parse_term(iter.by_ref())?;
    loop {
        match iter.peek().cloned() {
            Some('+') => {
                iter.next();
                ret += parse_term(iter.by_ref())?
            },
            Some('-') => {
                iter.next();
                ret -= parse_term(iter.by_ref())?
            }
            _ => break
        }
    }
    Ok(ret)
}

fn parse_term(iter: &mut Peekable<Chars>) -> Result<f64, ParseFloatError> {
    let mut ret = parse_num(iter.by_ref())?;
    loop {
        match iter.peek().cloned() {
            Some('×') => {
                iter.next();
                ret *= parse_num(iter.by_ref())?
            },
            Some('÷') => {
                iter.next();
                ret /= parse_num(iter.by_ref())?
            }
            _ => break
        }
    }
    Ok(ret)
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
        assert_eq!(calculate(Cow::Borrowed("-46")), Ok(-46.0));
    }

    #[test]
    fn evaluate_addition() {
        assert_eq!(calculate(Cow::Borrowed("15+25")), Ok(40.0));
    }

    #[test]
    fn evaluate_subtraction() {
        assert_eq!(calculate(Cow::Borrowed("34-16")), Ok(18.0));
    }

    #[test]
    fn evaluate_multiplication() {
        assert_eq!(calculate(Cow::Borrowed("34×4")), Ok(136.0))
    }

    #[test]
    fn evaluate_division() {
        assert_eq!(calculate(Cow::Borrowed("45÷5")), Ok(9.0));
    }

    #[test]
    fn evaluate_many_operations() {
        assert_eq!(calculate(Cow::Borrowed("48÷6+14-2×6+1")), Ok(11.0));
    }

    #[test]
    fn evaluate_many_operations_with_parenthesis() {
        assert_eq!(calculate(Cow::Borrowed("48÷(6+4)-(2×(6+1))")), Ok(-9.2));
    }
}
