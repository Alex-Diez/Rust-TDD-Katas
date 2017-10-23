use self::Number::{One, Two, Three, Four, Five, Six, Seven, Eight, Nine, Zero};

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
            _ => unreachable!("It is a bug!"),
        }
    }
}

#[derive(Default)]
pub struct Display {
    input: Option<&'static str>
}

impl Display {

    pub fn new() -> Display {
        Display {
            input: None
        }
    }

    pub fn output(&self) -> Option<Vec<Number>> {
        match self.input {
            Some(data) => Some(data.chars().map(Number::from).collect::<Vec<Number>>()),
            None => Some(vec![]),
        }
    }

    pub fn input(&mut self, data: &'static str) {
        self.input = Some(data);
    }
}
