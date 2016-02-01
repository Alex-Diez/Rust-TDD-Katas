use std::result::Result;
use std::num::ParseFloatError;
use std::iter::Peekable;
use std::str::Chars;
use std::option::Option;

pub fn evaluate(line: &str) -> Result<f32, ParseFloatError> {
    let mut iter = line.chars().peekable();
    Ok(try!(parse_expression(&mut iter.by_ref())))
}

pub fn parse_expression(iter: &mut Peekable<Chars>) -> Result<f32, ParseFloatError> {
    let mut accumulator = try!(parse_term(&mut iter.by_ref()));
    loop {
        accumulator = match next_symbol(&mut iter.by_ref()) {
            Some('+') => { try!(do_addition(accumulator, &mut iter.by_ref())) },
            Some('-') => { try!(do_subtitution(accumulator, &mut iter.by_ref())) },
            Some(_) | None => break,
        }
    }
    Ok(accumulator)
}

pub fn parse_term(iter: &mut Peekable<Chars>) -> Result<f32, ParseFloatError> {
    let mut accumulator = try!(parse_arg(&mut iter.by_ref()));
    loop {
        accumulator = match next_symbol(&mut iter.by_ref()) {
            Some('×') => { try!(do_multiplication(accumulator, &mut iter.by_ref())) },
            Some('÷') => { try!(do_division(accumulator, &mut iter.by_ref())) },
            Some(_) | None => break,
        }
    }
    Ok(accumulator)
}

fn parse_arg(iter: &mut Peekable<Chars>) -> Result<f32, ParseFloatError> {
    let mut value = vec![];
    loop {
        match next_symbol(&mut iter.by_ref()) {
            Some(c @ '0'...'9') | Some(c @ '.') => { skip_symbol(&mut iter.by_ref()); value.push(c) },
            Some('(') => {
                skip_symbol(&mut iter.by_ref());
                let result = try!(parse_expression(&mut iter.by_ref()));
                skip_symbol(&mut iter.by_ref());
                return Ok(result);
            },
            Some(_) | None => break,
        }
    }
    value.iter().map(|c| *c).collect::<String>().parse::<f32>()
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

fn do_subtitution(value: f32, iter: &mut Peekable<Chars>) -> Result<f32, ParseFloatError> {
    skip_symbol(&mut iter.by_ref());
    Ok(value - try!(parse_term(&mut iter.by_ref())))
}

fn do_multiplication(value: f32, iter: &mut Peekable<Chars>) -> Result<f32, ParseFloatError> {
    skip_symbol(&mut iter.by_ref());
    Ok(value * try!(parse_arg(&mut iter.by_ref())))
}

fn do_division(value: f32, iter: &mut Peekable<Chars>) -> Result<f32, ParseFloatError> {
    skip_symbol(&mut iter.by_ref());
    Ok(value / try!(parse_arg(&mut iter.by_ref())))
}
