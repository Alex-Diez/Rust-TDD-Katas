use std::str::Chars;
use std::option::Option;
use std::iter::Peekable;

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
        let mut chars = self.line.chars().peekable();
        let first_arg = Calculator::parse_arg(chars.by_ref());
        println!("first_arg = {:?}", first_arg);
        let sign = Calculator::parse_sign(chars.by_ref());
        println!("sign = {:?}", sign);
        let second_arg = Calculator::parse_arg(chars.by_ref());
        println!("second_arg = {:?}", second_arg);
        match sign {
            Some('+') => first_arg + second_arg,
            Some('-') => first_arg - second_arg,
            _ => first_arg
        }
    }

    fn parse_arg(chars: &mut Peekable<Chars>) -> u32 {
        let mut acc = 0;
        while let Some(d) = chars.peek().cloned().and_then(|d| d.to_digit(10)) {
            acc += acc * 10 + d;
            chars.next();
        }
        acc
    }

    fn parse_sign(chars: &mut Peekable<Chars>) -> Option<char> {
        chars.next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_diff_numbers() {
        let calc1 = Calculator::new("1");
        let calc2 = Calculator::new("2");

        assert_eq!(calc1.evaluate(), 1);
        assert_eq!(calc2.evaluate(), 2);
    }

    #[test]
    fn test_evaluate_one_plus_two() {
        let calc = Calculator::new("1+2");

        assert_eq!(calc.evaluate(), 3);
    }

    #[test]
    fn test_evaluate_two_minus_one() {
        let calc = Calculator::new("2-1");

        assert_eq!(calc.evaluate(), 1);
    }
}
