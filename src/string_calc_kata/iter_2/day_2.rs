use std::str::Chars;
use std::iter::Peekable;
use std::result::Result;
use std::num::ParseFloatError;

pub fn evaluate(src: &str) -> Result<f64, ParseFloatError> {
    let mut iter = src.chars().peekable();
    let mut result = try!(parse_term(iter.by_ref()));
    while iter.peek().is_some() {
        match iter.next() {
            Some('+') => result += try!(parse_term(iter.by_ref())),
            Some('-') => result -= try!(parse_term(iter.by_ref())),
            Some(_) | None => break,
        }
    }
    Ok(result)
}

fn parse_term(iter: &mut Peekable<Chars>) -> Result<f64, ParseFloatError> {
    let mut result = try!(parse_arg(iter.by_ref()));
    while iter.peek().is_some() {
        match iter.peek().cloned() {
            Some('×') => {
                iter.next();
                result *= try!(parse_arg(iter.by_ref()));
            },
            Some('÷') => {
                iter.next();
                result /= try!(parse_arg(iter.by_ref()));
            }
            Some(_) | None => break,
        }
    }
    Ok(result)
}

fn parse_arg(iter: &mut Peekable<Chars>) -> Result<f64, ParseFloatError> {
    let mut number = Vec::new();
    while iter.peek().is_some() {
        match iter.peek().cloned() {
            Some(digit @ '0'...'9') => {
                number.push(digit);
                iter.next();
            }
            Some(point @ '.') => {
                number.push(point);
                iter.next();
            },
            Some(_) | None => break,
        }
    }
    number.iter().cloned().collect::<String>().parse::<f64>()
}
