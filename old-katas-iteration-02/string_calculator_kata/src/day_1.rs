use std::borrow::Cow;
use std::iter::Peekable;
use std::num::ParseFloatError;
use std::str::Chars;

pub fn calculate<'s>(src: Cow<'s, str>) -> Result<f64, ParseFloatError> {
    let mut chars = src.chars().peekable();
    let mut ret = parse_number(chars.by_ref())?;
    loop {
        match chars.peek().cloned() {
            Some('+') => {
                chars.next();
                ret += parse_number(chars.by_ref())?
            }
            Some('-') => {
                chars.next();
                ret -= parse_number(chars.by_ref())?
            }
            Some(_) | None => break
        }
        chars.next();
    };
    Ok(ret)
}

fn parse_number(iter: &mut Peekable<Chars>) -> Result<f64, ParseFloatError> {
    let mut number = String::new();
    loop {
        match iter.peek().cloned() {
            Some('+') | None => break,
            Some('-') => {
                if number.is_empty() {
                    number.push('-')
                } else {
                    break
                }
            }
            Some(d) => number.push(d),
        }
        iter.next();
    }
    number.parse::<f64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluate_negative_number() {
        assert_eq!(calculate(Cow::Borrowed("-20")), Ok(-20.0));
    }

    #[test]
    fn evaluate_addition() {
        assert_eq!(calculate(Cow::Borrowed("14+5")), Ok(19.0))
    }

    #[test]
    fn evaluate_subtraction() {
        assert_eq!(calculate(Cow::Borrowed("5-21")), Ok(-16.0));
    }
}
