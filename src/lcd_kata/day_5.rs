use std::str::Chars;

use self::Number::{One, Two, Three, Four, Five, Six, Seven, Eight, Nine, Zero};
use self::Data::{NotANumber, Output};

#[derive(PartialEq, Debug)]
pub enum Data {
    Output(Vec<Number>),
    NotANumber
}

impl From<Chars<'static>> for Data {

    fn from(chars: Chars<'static>) -> Data {
        let mut vec = Vec::with_capacity(chars.as_str().len());
        for c in chars {
            if c < '0' || c > '9' {
                return NotANumber;
            }
            vec.push(Number::from(c));
        }
        Output(vec)
    }
}

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
    Zero
}

impl From<char> for Number {

    fn from(c: char) -> Number {
        match c {
            '1' => One,
            '2' => Two,
            '3' => Three,
            '4' => Four,
            '5' => Five,
            '6' => Six,
            '8' => Eight,
            '7' => Seven,
            '9' => Nine,
            '0' => Zero,
            _ => unreachable!("BUG"),
        }
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

    pub fn output(&self) -> Data {
        match self.input {
            Some(data) => Data::from(data.chars()),
            None => Output(vec![]),
        }
    }
}
