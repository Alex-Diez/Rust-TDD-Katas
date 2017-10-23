use std::str::Chars;
use std::iter::Peekable;
use std::result::Result;
use std::num::ParseFloatError;

pub fn evaluate(src: &str) -> Result<f32, ParseFloatError> {
    let mut iter = src.chars().peekable();
    let mut result = try!(parse_term(iter.by_ref()));
    while iter.peek().is_some() {
        match iter.next() {
            Some('+') => { result += try!(parse_term(iter.by_ref())); },
            Some('-') => { result -= try!(parse_term(iter.by_ref())); },
            Some(_) | None => break,
        }
    }
    Ok(result)
}

fn parse_term(iter: &mut Peekable<Chars>) -> Result<f32, ParseFloatError> {
    let mut data = Vec::new();
    while iter.peek().is_some() {
        match iter.peek().cloned() {
            Some(digit @ '0'...'9') => {
                data.push(digit);
                iter.next();
            },
            Some(point @ '.') => {
                data.push(point);
                iter.next();
            },
            Some(_) | None => break,
        }
    }
    data.iter().cloned().collect::<String>().parse::<f32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_evaluate_float_number() {
        assert_eq!(evaluate("32435.246"), Ok(32435.246));
    }

    #[test]
    fn should_evaluate_add_operation() {
        assert_eq!(evaluate("35435.657+213.546"), Ok(35649.203));
    }

    #[test]
    fn should_evaluate_sub_operation() {
        assert_eq!(evaluate("3465.6757-324.2346"), Ok(3141.4411));
    }
}
