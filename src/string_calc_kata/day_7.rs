use std::result::Result;
use std::num::ParseFloatError;
use std::str::Chars;
use std::iter::Peekable;

pub fn evaluate(line: &str) -> Result<f64, ParseFloatError> {
    let mut iter = line.chars().peekable();
    let mut accumulator = try!(parse_term(&mut iter.by_ref()));
    loop {
        match iter.peek() {
            Some(&'+') => { iter.next(); accumulator += try!(parse_term(&mut iter.by_ref())) },
            Some(&'-') => { iter.next(); accumulator -= try!(parse_term(&mut iter.by_ref())) },
            Some(_) | None => break,
        }
    }
    Ok(accumulator)
}

fn parse_term(iter: &mut Peekable<Chars>) -> Result<f64, ParseFloatError> {
    let mut accumulator = try!(parse_factor(&mut iter.by_ref()));
    loop {
        match iter.peek() {
            Some(&'×') => { iter.next(); accumulator *= try!(parse_factor(&mut iter.by_ref())) },
            Some(&'÷') => { iter.next(); accumulator /= try!(parse_factor(&mut iter.by_ref())) },
            Some(_) | None => break,
        }
    }
    Ok(accumulator)
}

fn parse_factor(iter: &mut Peekable<Chars>) -> Result<f64, ParseFloatError> {
    let mut s = vec![];
    loop {
        match iter.next() {
            Some(v @ '0'...'9') | Some(v @ '.') => s.push(v),
            Some('(') => return evaluate(&iter.take_while(|c| c == &')').collect::<String>()),
            Some(_) | None => break,
        }
    }
    s.iter().map(|c| *c).collect::<String>().parse::<f64>()
}
