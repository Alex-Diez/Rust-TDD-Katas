use std::option::Option;
use std::result::Result;
use std::error::Error;
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
            c => panic!("{:?} is not a number", c),
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
        "display can't display current input"
    }
}

impl fmt::Display for DisplayError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Display Error")
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

    pub fn input(&mut self, data: &'static str) {
        self.input = Some(data);
    }

    pub fn output(&self) -> DisplayResult {
        match self.input {
            Some(data) => {
                Ok(data.chars().map(Number::from).collect::<Vec<Number>>())
            },
            None => Err(DisplayError::new()),
        }
    }
}
