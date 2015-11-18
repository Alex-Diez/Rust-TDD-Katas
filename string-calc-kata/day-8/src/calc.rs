use std::result::Result;
use std::num::ParseFloatError;
use std::str::Chars;
use std::iter::Peekable;

pub fn evaluate(line: &str) -> Result<f32, ParseFloatError> {
    let mut iter = line.chars().peekable();
    Ok(try!(parse_expression(&mut iter)))
}

fn parse_expression(iter: &mut Peekable<Chars>) -> Result<f32, ParseFloatError> {
    let mut accumulator = try!(parse_term(&mut iter.by_ref()));
    loop {
        let sign = iter.peek().map(|c| *c);
        match sign {
            Some('+') => { iter.next(); accumulator += try!(parse_term(&mut iter.by_ref())) },
            Some('-') => { iter.next(); accumulator -= try!(parse_term(&mut iter.by_ref())) },
            Some(_) | None => break,
        }
    }
    Ok(accumulator)
}

fn parse_term(iter: &mut Peekable<Chars>) -> Result<f32, ParseFloatError> {
    let mut accumulator = try!(parse_factor(iter.by_ref()));
    loop {
        let sign = iter.peek().map(|c| *c);
        match sign {
            Some('×') => { iter.next(); accumulator *= try!(parse_factor(&mut iter.by_ref())) },
            Some('÷') => { iter.next(); accumulator /= try!(parse_factor(&mut iter.by_ref())) },
            Some(_) | None => break,
        }
    }
    Ok(accumulator)
}

fn parse_factor(iter: &mut Peekable<Chars>) -> Result<f32, ParseFloatError> {
    let mut value = vec![];
    loop {
        let symbol = iter.peek().map(|c| *c);
        match symbol {
            Some(c @ '0'...'9') | Some(c @ '.') => { iter.next(); value.push(c) },
            Some(_) | None => break,
        }
    }
    value.iter().map(|c| *c).collect::<String>().parse::<f32>()
}
