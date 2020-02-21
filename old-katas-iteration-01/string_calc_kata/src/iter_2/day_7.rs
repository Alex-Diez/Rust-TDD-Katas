use std::iter::Peekable;
use std::str::Chars;
use std::result::Result;
use std::num::ParseFloatError;

pub fn evaluate(src: &str) -> Result<f64, ParseFloatError> {
    let mut iter = src.chars().peekable();
    parse_expression(iter.by_ref())
}

fn parse_expression(iter: &mut Peekable<Chars>) -> Result<f64, ParseFloatError> {
    let mut result = try!(parse_term(iter.by_ref()));
    loop {
        match iter.peek().cloned() {
            Some('+') => {
                iter.next();
                result += try!(parse_term(iter.by_ref()));
            },
            Some('-') => {
                iter.next();
                result -= try!(parse_term(iter.by_ref()));
            },
            Some(_) | None => break,
        }
    }
    Ok(result)
}

fn parse_term(iter: &mut Peekable<Chars>) -> Result<f64, ParseFloatError> {
    let mut result = try!(parse_arg(iter.by_ref()));
    loop {
        match iter.peek().cloned() {
            Some('×') => {
                iter.next();
                result *= try!(parse_term(iter.by_ref()))
            },
            Some('÷') => {
                iter.next();
                result /= try!(parse_term(iter.by_ref()))
            },
            Some(_) | None => break,
        }
    }
    Ok(result)
}

fn parse_arg(iter: &mut Peekable<Chars>) -> Result<f64, ParseFloatError> {
    if iter.peek() == Some(&'(') {
        iter.next();
        let ret = parse_expression(iter.by_ref());
        iter.next();
        ret
    } else {
        let mut num = Vec::new();
        loop {
            match iter.peek().cloned() {
                Some(digit @ '0' ... '9') => {
                    iter.next();
                    num.push(digit);
                },
                Some(point @ '.') => {
                    iter.next();
                    num.push(point);
                },
                Some(_) | None => break,
            }
        }
        let term = num.drain(..).collect::<String>();
        term.parse::<f64>()
    }
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

    #[test]
    fn should_evaluate_mul_operation() {
        assert_eq!(evaluate("354.76×25.2"), Ok(8939.952));
    }

    #[test]
    fn should_evaluate_div_operation() {
        assert_eq!(evaluate("3254.546÷32.32"), Ok(100.69758663366336));
    }

    #[test]
    fn should_evaluate_sequence_of_operations() {
        assert_eq!(evaluate("3254+324×23-461.125×2+4.248÷23-461×1.25+48"), Ok(9255.684695652173));
    }

    #[test]
    fn should_evaluate_expression_with_parenthesis() {
        assert_eq!(evaluate("3425+214+(213.3-22.4×12)-3254×(234.2+32.2)+54÷2"), Ok(-863255.1));
    }
}
