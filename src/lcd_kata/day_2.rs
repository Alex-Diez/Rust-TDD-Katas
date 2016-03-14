#![allow(new_without_default)]

use std::option::Option;
use std::error::Error;
use std::result::Result;
use std::fmt;

#[derive(PartialEq, Debug)]
pub enum Number {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Zero,
    NotANumber
}

impl From<char> for Number {

    fn from(c: char) -> Number {
        match c {
            '1' => Number::One,
            '2' => Number::Two,
            '3' => Number::Three,
            '4' => Number::Four,
            '5' => Number::Five,
            '6' => Number::Six,
            '7' => Number::Seven,
            '8' => Number::Eight,
            '9' => Number::Nine,
            '0' => Number::Zero,
            _ => Number::NotANumber,
        }
    }
}

pub type DisplayResult = Result<Vec<Number>, DisplayError>;

#[derive(Debug)]
pub struct DisplayError;

impl DisplayError {

    pub fn new() -> DisplayError {
        DisplayError
    }
}

impl Error for DisplayError {

    fn description(&self) -> &str {
        "LCD can't display current input"
    }
}

impl fmt::Display for DisplayError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LCD error")
    }
}

pub struct Display {
    input: Option<&'static str>
}

impl Display {

    pub fn new() -> Display {
        Display {
            input: None
        }
    }

    pub fn output(&self) -> DisplayResult {
        match self.input {
            Some(s) => {
                let mut v = Vec::with_capacity(s.len());
                for c in s.chars() {
                    let n = Number::from(c);
                    if n == Number::NotANumber {
                        return Err(DisplayError::new());
                    }
                    else {
                        v.push(n);
                    }
                }
                Ok(v)
            },
            None => Ok(vec![]),
        }
    }

    pub fn input(&mut self, data: &'static str) {
        self.input = Some(data);
    }
}
