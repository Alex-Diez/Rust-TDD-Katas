use std::result::Result;
use std::num::ParseFloatError;
use std::iter::Peekable;
use std::iter::Iterator;
use std::str::Chars;

pub fn evaluate(line: &str) -> Result<f64, ParseFloatError> {
    let mut iter = line.chars().peekable();
    Ok(try!(parse_expression(&mut iter.by_ref())))
}

fn parse_expression(iter: &mut Peekable<Chars>) -> Result<f64, ParseFloatError> {
    let mut accumulator = try!(parse_term(&mut iter.by_ref()));
    loop {
        accumulator = match next_symbol(&mut iter.by_ref()) {
            Some('+') => try!(do_addition(accumulator, &mut iter.by_ref())),
            Some('-') => try!(do_substitution(accumulator, &mut iter.by_ref())),
            Some(_) | None => break,
        }
    }
    Ok(accumulator)
}

fn parse_term(iter: &mut Peekable<Chars>) -> Result<f64, ParseFloatError> {
    let mut accumulator = try!(parse_arg(&mut iter.by_ref()));
    loop {
        accumulator = match next_symbol(&mut iter.by_ref()) {
            Some('×') => try!(do_multiplication(accumulator, &mut iter.by_ref())),
            Some('÷') => try!(do_division(accumulator, &mut iter.by_ref())),
            Some(_) | None => break,
        }
    }
    Ok(accumulator)
}

fn parse_arg(iter: &mut Peekable<Chars>) -> Result<f64, ParseFloatError> {
    let mut value = vec![];
    loop {
        let symbol = iter.peek().map(|c| *c);
        match symbol {
            Some(c @ '0'...'9') | Some(c @ '.') => { iter.next(); value.push(c) },
            Some('(') => { iter.next(); let result = try!(parse_expression(&mut iter.by_ref())); iter.next(); return Ok(result) },
            Some(_) | None => { break }
        }
    }
    value.iter().map(|c| *c).collect::<String>().parse::<f64>()
}

fn next_symbol(iter: &mut Peekable<Chars>) -> Option<char> {
    iter.peek().map(|c| *c)
}

fn skip_symbol(iter: &mut Peekable<Chars>) {
    iter.next();
}

fn do_addition(value: f64, iter: &mut Peekable<Chars>) -> Result<f64, ParseFloatError> {
    skip_symbol(&mut iter.by_ref());
    Ok(value + try!(parse_term(&mut iter.by_ref())))
}

fn do_substitution(value: f64, iter: &mut Peekable<Chars>) -> Result<f64, ParseFloatError> {
    skip_symbol(&mut iter.by_ref());
    Ok(value - try!(parse_term(&mut iter.by_ref())))
}

fn do_multiplication(value: f64, iter: &mut Peekable<Chars>) -> Result<f64, ParseFloatError> {
    skip_symbol(&mut iter.by_ref());
    Ok(value * try!(parse_arg(&mut iter.by_ref())))
}

fn do_division(value: f64, iter: &mut Peekable<Chars>) -> Result<f64, ParseFloatError> {
    skip_symbol(&mut iter.by_ref());
    Ok(value / try!(parse_arg(&mut iter.by_ref())))
}
