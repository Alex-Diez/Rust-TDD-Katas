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
            Some(_) | None => break
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
            Some(_) | None => break
        }
    }
    Ok(ret)
}

fn parse_num(iter: &mut Peekable<Chars>) -> Result<f64, ParseFloatError> {
    let mut num = String::new();
    loop {
        match iter.peek().cloned() {
            Some('+') | Some('×') | Some('÷') | Some(')') => break,
            Some('-') if num.is_empty() => num.push('-'),
            Some(d @ '0'...'9') => num.push(d),
            Some('(') => {
                iter.next();
                let ret = parse_expression(iter.by_ref())?;
                if let Some(')') = iter.peek().cloned() {
                    iter.next();
                }
                return Ok(ret);
            }
            Some(_) | None => break,
        }
        iter.next();
    }
    num.parse::<f64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluate_negative_number() {
        assert_eq!(calculate(Cow::Borrowed("-22")), Ok(-22.0));
    }

    #[test]
    fn evaluate_addition() {
        assert_eq!(calculate(Cow::Borrowed("23+24")), Ok(47.0));
    }

    #[test]
    fn evaluate_subtraction() {
        assert_eq!(calculate(Cow::Borrowed("25-35")), Ok(-10.0));
    }

    #[test]
    fn evaluate_multiplication() {
        assert_eq!(calculate(Cow::Borrowed("4×5")), Ok(20.0));
    }

    #[test]
    fn evaluate_division() {
        assert_eq!(calculate(Cow::Borrowed("40÷8")), Ok(5.0));
    }

    #[test]
    fn evaluate_many_operations() {
        assert_eq!(calculate(Cow::Borrowed("4+3×6-20+15÷5-2+11")), Ok(14.0));
    }

    #[test]
    fn evaluate_operation_with_parenthesis() {
        assert_eq!(calculate(Cow::Borrowed("(4+3)×6-(20+10)÷(5-2)+11")), Ok(43.0));
    }
}
