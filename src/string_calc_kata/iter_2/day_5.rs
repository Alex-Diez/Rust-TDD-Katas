#![allow(new_without_default)]

use std::iter::Peekable;
use std::str::Chars;
use std::result::Result;
use std::num::ParseFloatError;

pub struct Calculator;
/*
add: Box::new(
    |acc, iter| {
        iter.next();
        Ok(acc + try!(self.parse_term(iter.by_ref())))
    }
),
sub: Box::new(
    |acc, iter| {
        iter.next();
        Ok(acc - try!(self.parse_term(iter.by_ref())))
    }
),
mul: Box::new(
    |acc, iter| {
        iter.next();
        result * try!(self.parse_arg(iter.by_ref()))
    }
),
div: Box::new(
    |acc, iter| {
        iter.next();
        acc / try!(self.parse_arg(iter.by_ref()))
    }
),
parse_expression: Box::new(
    |iter| {
        let mut result = try!(self.parse_term(iter.by_ref()));
        loop {
            match iter.peek().cloned() {
                Some('+') => result = self.add(result, iter.by_ref()),
                Some('-') => result = self.sub(result, iter.by_ref()),
                Some(_) | None => break,
            }
        }
        Ok(result)
    }
),
parse_term: Box::new(
    |iter| {
        let mut result = try!(self.parse_arg(iter.by_ref()));
        loop {
            match iter.peek().cloned() {
                Some('×') => result = self.mul(result, iter.by_ref()),
                Some('÷') => result = self.div(result, iter.by_ref()),
                Some(_) | None => break,
            }
        }
        Ok(result)
    }
),
parse_arg: Box::new(
    |iter| {
        let mut num = Vec::new();
        loop {
            match iter.peek().cloned() {
                Some(digit @ '0'...'9') => { iter.next(); num.push(digit); },
                Some(point @ '.') => { iter.next(); num.push(point); },
                Some('(') => {
                    iter.next();
                    let result = try!(self.parse_expression(iter.by_ref()));
                    iter.next();
                    return Ok(result);
                },
                Some(_) | None => break,
            }
        }
        let s = num.drain(..).collect::<String>();
        s.parse::<f64>()
    }
)
*/

impl Calculator {

    pub fn new() -> Calculator {
        Calculator
    }

    pub fn evaluate(&self, src: &str) -> Result<f64, ParseFloatError> {
        let mut iter = src.chars().peekable();
        self.parse_expression(&mut iter)
    }

    fn parse_expression(&self, iter: &mut Peekable<Chars>) -> Result<f64, ParseFloatError> {
        let mut result = try!(self.parse_term(iter.by_ref()));
        loop {
            match iter.peek().cloned() {
                Some('+') => { iter.next(); result += try!(self.parse_term(iter.by_ref())); },
                Some('-') => { iter.next(); result -= try!(self.parse_term(iter.by_ref())); },
                Some(_) | None => break,
            }
        }
        Ok(result)
    }

    fn parse_term(&self, iter: &mut Peekable<Chars>) -> Result<f64, ParseFloatError> {
        let mut result = try!(self.parse_arg(iter.by_ref()));
        loop {
            match iter.peek().cloned() {
                Some('×') => { iter.next(); result *= try!(self.parse_arg(iter.by_ref())); },
                Some('÷') => { iter.next(); result /= try!(self.parse_arg(iter.by_ref())); },
                Some(_) | None => break,
            }
        }
        Ok(result)
    }

    fn parse_arg(&self, iter: &mut Peekable<Chars>) -> Result<f64, ParseFloatError> {
        let mut num = Vec::new();
        loop {
            match iter.peek().cloned() {
                Some(digit @ '0'...'9') => { iter.next(); num.push(digit); },
                Some(point @ '.') => { iter.next(); num.push(point); },
                Some('(') => {
                    iter.next();
                    let result = try!(self.parse_expression(iter.by_ref()));
                    iter.next();
                    return Ok(result);
                },
                Some(_) | None => break,
            }
        }
        let s = num.drain(..).collect::<String>();
        s.parse::<f64>()
    }
}
