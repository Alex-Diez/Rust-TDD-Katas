use std::result::Result;
use std::error::Error;
use std::str::FromStr;
use std::fmt;

use self::Digit::{One, Two, Three, Four, Five, Six, Seven, Eight, Nine, Zero};

#[derive(PartialEq, Debug)]
pub enum Digit {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Zero
}

impl From<char> for Digit {

    fn from(c: char) -> Digit {
        match c {
            '1' => One,
            '2' => Two,
            '3' => Three,
            '4' => Four,
            '5' => Five,
            '6' => Six,
            '7' => Seven,
            '8' => Eight,
            '9' => Nine,
            '0' => Zero,
            _ => unreachable!("BUG"),
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct VecDigit {
    vec: Vec<Digit>
}

impl VecDigit {

    pub fn empty() -> VecDigit {
        VecDigit::new(vec![])
    }

    pub fn new(vec: Vec<Digit>) -> VecDigit {
        VecDigit {
            vec: vec
        }
    }
}

impl FromStr for VecDigit {
    type Err = DisplayError;

    fn from_str(s: &str) -> Result<VecDigit, DisplayError> {
        let mut vec = Vec::with_capacity(s.len());
        for c in s.chars() {
            if c < '0' || c > '9' {
                return Err(DisplayError::new());
            }
            vec.push(Digit::from(c));
        }
        Ok(VecDigit::new(vec))
    }
}

#[derive(Debug)]
pub struct DisplayError;

impl DisplayError {

    fn new() -> DisplayError {
        DisplayError
    }
}

impl Error for DisplayError {

    fn description(&self) -> &str {
        "Display Error"
    }
}

impl fmt::Display for DisplayError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "display error")
    }
}

#[derive(Default)]
pub struct Display {
    data: Option<&'static str>
}

impl Display {

    pub fn new() -> Display {
        Display {
            data: None
        }
    }

    pub fn output(&self) -> Result<VecDigit, DisplayError> {
        match self.data {
            Some(data) => VecDigit::from_str(data),
            None => Ok(VecDigit::empty()),
        }
    }

    pub fn input(&mut self, data: &'static str) {
        self.data = Some(data);
    }
}
