use std::iter::Peekable;
use std::str::Chars;
// use std::collections::HashMap;
use std::num::ParseFloatError;
use std::result::Result;

pub fn evaluate(src: &str) -> Result<f64, ParseFloatError> {
    let mut iter = src.chars().peekable();
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
    let mut number = Vec::new();
    loop {
        match iter.peek().cloned() {
            Some(digit @ '0'...'9') => { iter.next(); number.push(digit); },
            Some(point @ '.') => { iter.next(); number.push(point); },
            Some(_) | None => break,
        }
    }
    number.iter().cloned().collect::<String>().parse::<f64>()
}

/*
struct Calc<'c> {
    operations: HashMap<char, Box<Fn(f64, &mut Peekable<Chars>) -> Result<f64, ParseFloatError> + 'c>>
}

impl <'c> Calc <'c> {

    fn new() -> Calc<'c> {
        let mut operations: HashMap<char, Box<Fn(f64, &mut Peekable<Chars>) -> Result<f64, ParseFloatError>>> = HashMap::with_capacity(4);
        for _ in 0..4 {
            operations.insert('+', Box::new(|acc, iter| { iter.next(); Ok(acc + try!(parse_term(iter.by_ref()))) } ) );
            operations.insert('-', Box::new(|acc, iter| { iter.next(); Ok(acc - try!(parse_term(iter.by_ref()))) } ) );
            operations.insert('×', Box::new(|acc, iter| { iter.next(); Ok(acc * try!(parse_arg(iter.by_ref()))) } ) );
            operations.insert('÷', Box::new(|acc, iter| { iter.next(); Ok(acc / try!(parse_arg(iter.by_ref()))) } ) );
        }
        Calc {
            operations: operations
        }
    }

    fn operation<F>(&self, op: char) -> Option<F>
            where F: Fn(f64, &mut Peekable<Chars>) -> f64 {
        self.operations.get(&op).map(|f| **f)
    }
}
*/
