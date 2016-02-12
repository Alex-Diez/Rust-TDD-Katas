use std::str::Chars;

use self::DisplayResult::{Output, NotANumber};
use self::Number::{One, Two, Three, Four, Five, Six, Seven, Eight, Nine, Zero};

#[derive(PartialEq, Debug)]
pub enum DisplayResult {
    Output(Vec<Number>),
    NotANumber
}

impl From<Chars<'static>> for DisplayResult {

    fn from(chars: Chars) -> DisplayResult {
        let len = chars.as_str().len();
        let mut vec = Vec::with_capacity(len);
        for c in chars {
            if c < '0' || c > '9' {
                return NotANumber;
            }
            let n = Number::from(c);
            vec.push(n);
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
            '7' => Seven,
            '8' => Eight,
            '9' => Nine,
            '0' => Zero,
            _ => unreachable!("Impossible!"),
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

    pub fn output(&self) -> DisplayResult {
        match self.input {
            Some(data) => DisplayResult::from(data.chars()),
            None => Output(vec![]),
        }
    }

    pub fn input(&mut self, data: &'static str) {
        self.input = Some(data);
    }
}
