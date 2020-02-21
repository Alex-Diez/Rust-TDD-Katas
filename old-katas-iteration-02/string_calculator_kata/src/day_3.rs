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
            },
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
            },
            _ => break
        }
    }
    Ok(ret)
}

fn parse_num(iter: &mut Peekable<Chars>) -> Result<f64, ParseFloatError> {
    let mut num = String::new();
    loop {
        match iter.peek().cloned() {
            Some('+') | Some('×') | Some('÷') | Some(')') => break,
            Some('-') if !num.is_empty() => break,
            Some('(') => {
                iter.next();
                let ret = parse_expression(iter.by_ref());
                if let Some(')') = iter.peek().cloned() {
                    iter.next();
                }
                return ret;
            },
            Some(c) => num.push(c),
            _ => break
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
        assert_eq!(calculate(Cow::Borrowed("-17.0")), Ok(-17.0));
    }
    
    #[test]
    fn evaluate_addition() {
        assert_eq!(calculate(Cow::Borrowed("14+6")), Ok(20.0));
    }

    #[test]
    fn evaluate_subtraction() {
        assert_eq!(calculate(Cow::Borrowed("24-31")), Ok(-7.0));
    }

    #[test]
    fn evaluate_multiplication() {
        assert_eq!(calculate(Cow::Borrowed("23×5")), Ok(115.0));
    }

    #[test]
    fn evaluate_division() {
        assert_eq!(calculate(Cow::Borrowed("44÷4")), Ok(11.0));
    }

    #[test]
    fn evaluate_many_operations() {
        assert_eq!(calculate(Cow::Borrowed("4-20÷5+11×2-48÷6+1")), Ok(15.0));
    }

    #[test]
    fn evaluate_operations_with_parenthesis() {
        assert_eq!(calculate(Cow::Borrowed("(4-20)÷4+11×(2+48)÷5-1")), Ok(105.0))
    }
}
