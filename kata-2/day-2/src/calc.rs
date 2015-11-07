use std::str::Chars;
use std::iter::Peekable;
use std::option::Option;

pub struct Calculator<'a> {
    line: &'a str
}

impl <'a> Calculator<'a> {

    pub fn new(line: &'a str) -> Calculator {
        Calculator {
            line: line
        }
    }

    pub fn evaluate(&self) -> u32 {
        let mut chars = self.line.chars();
        println!("chars - {:?}", chars.as_str());
        let first_arg = Calculator::parse_arg(&mut chars);
        println!("chars - {:?}", chars.as_str());
        let sign = Calculator::parse_sign(&mut chars);
        println!("chars - {:?}", chars.as_str());
        let second_arg = Calculator::parse_arg(&mut chars);
        println!("operation - {:?} {:?} {:?}", first_arg, sign, second_arg);
        match sign {
            Some('+') => first_arg + second_arg,
            Some('-') => first_arg - second_arg,
            Some(_) => 0,
            None => first_arg
        }
    }

    fn parse_arg(chars: &mut Chars) -> u32 {
        let mut peekable = chars.by_ref().peekable();
        let mut acc = 0;
        while peekable.peek().is_some() && (*peekable.peek().unwrap()).is_digit(10) {
            let value = (*peekable.peek().unwrap()).to_digit(10).unwrap();
            acc += acc * 10 + value;
            peekable.next();
        }
        acc
    }

    fn parse_sign(chars: &mut Chars) -> Option<char> {
        chars.next()
    }
}
