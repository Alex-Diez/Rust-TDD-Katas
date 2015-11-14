use std::result::Result;
use std::num::ParseFloatError;
use std::str::Chars;
use std::iter::Peekable;
use std::option::Option;

pub fn evaluate(line: &str) -> Result<f32, ParseFloatError> {
    Ok(try!(parse_expression(&mut line.chars().peekable())))
}

fn parse_expression(iter: &mut Peekable<Chars>) -> Result<f32, ParseFloatError> {
    let mut value = try!(parse_term(&mut iter.by_ref()));
    loop {
        value = match retrieve_sign(&mut iter.by_ref()) {
            Some('+') => try!(do_addition(value, &mut iter.by_ref())),
            Some('-') => try!(do_subtraction(value, &mut iter.by_ref())),
            Some(_) | None => break,
        }
    }
    Ok(value)
}

fn parse_term(iter: &mut Peekable<Chars>) -> Result<f32, ParseFloatError> {
    let mut value = try!(parse_number(&mut iter.by_ref()));
    loop {
        value = match retrieve_sign(&mut iter.by_ref()) {
            Some('×') => try!(do_multiplation(value, &mut iter.by_ref())),
            Some('÷') => try!(do_division(value, &mut iter.by_ref())),
            Some(_) | None => break,
        }
    }
    Ok(value)
}

fn parse_number(iter: &mut Peekable<Chars>) -> Result<f32, ParseFloatError> {
    let mut value = vec![];
    loop {
        match next_symbol(&mut iter.by_ref()) {
            Some(c @ '0'...'9') | Some(c @ '.') => { skip_symbol(&mut iter.by_ref()); value.push(c) },
            Some('(') => { skip_symbol(&mut iter.by_ref()); let result = Ok(try!(parse_expression(&mut iter.by_ref()))); iter.next(); return result },
            Some(_) | None => break,
        }
    }
    value.iter().map(|c| *c).collect::<String>().parse::<f32>()
}

fn retrieve_sign(iter: &mut Peekable<Chars>) -> Option<char> {
    next_symbol(&mut iter.by_ref())
}

fn next_symbol(iter: &mut Peekable<Chars>) -> Option<char> {
    iter.peek().map(|c| *c)
}

fn skip_symbol(iter: &mut Peekable<Chars>) {
    iter.next();
}

fn do_addition(value: f32, iter: &mut Peekable<Chars>) -> Result<f32, ParseFloatError> {
    skip_symbol(&mut iter.by_ref());
    Ok(value + try!(parse_term(&mut iter.by_ref())))
}

fn do_subtraction(value: f32, iter: &mut Peekable<Chars>) -> Result<f32, ParseFloatError> {
    skip_symbol(&mut iter.by_ref());
    Ok(value - try!(parse_term(&mut iter.by_ref())))
}

fn do_multiplation(value: f32, iter: &mut Peekable<Chars>) -> Result<f32, ParseFloatError> {
    skip_symbol(&mut iter.by_ref());
    Ok(value * try!(parse_number(&mut iter.by_ref())))
}

fn do_division(value: f32, iter: &mut Peekable<Chars>) -> Result<f32, ParseFloatError> {
    skip_symbol(&mut iter.by_ref());
    Ok(value / try!(parse_number(&mut iter.by_ref())))
}
