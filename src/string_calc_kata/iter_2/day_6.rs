use std::str::Chars;
use std::iter::Peekable;
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
            Some('+') => { iter.next(); result += try!(parse_term(iter.by_ref())); },
            Some('-') => { iter.next(); result -= try!(parse_term(iter.by_ref())); },
            Some(_) | None => break,
        }
    }
    Ok(result)
}

fn parse_term(iter: &mut Peekable<Chars>) -> Result<f64, ParseFloatError> {
    let mut result = try!(parse_arg(iter.by_ref()));
    loop {
        match iter.peek().cloned() {
            Some('×') => { iter.next(); result *= try!(parse_arg(iter.by_ref())); },
            Some('÷') => { iter.next(); result /= try!(parse_arg(iter.by_ref())); },
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
    }
    else {
        let mut num = Vec::new();
        loop {
            match iter.peek().cloned() {
                Some(digit @ '0'...'9') => { iter.next(); num.push(digit); },
                Some(point @ '.') => { iter.next(); num.push(point); },
                Some(_) | None => break,
            }
        }
        let arg = num.drain(..).collect::<String>();
        arg.parse::<f64>()
    }
}
