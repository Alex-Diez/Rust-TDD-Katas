use std::borrow::Cow;
use std::iter::Peekable;
use std::num::ParseFloatError;
use std::str::Chars;

pub struct Calculator<'s> {
    src: Cow<'s, str>
}

impl <'s> Calculator <'s> {
    pub fn calculate(&'s self) -> Result<f64, ParseFloatError> {
        let iter = self.src.chars().peekable();
        let mut parser = Parser::new(iter);
        parser.parse_expression()
    }

    pub fn new(src: Cow<'s, str>) -> Self {
        Calculator { src }
    }
}

struct Parser<'p> {
    iter: Peekable<Chars<'p>>
}

impl <'p> Parser<'p> {
    fn new(iter: Peekable<Chars<'p>>) -> Self {
        Parser { iter }
    }

    fn parse_expression(&mut self) -> Result<f64, ParseFloatError> {
        let mut ret = self.parse_term();
        loop {
            let op = match self.next() {
                Some('+') => {
                    self.skip();
                    add
                },
                Some('-') => {
                    self.skip();
                    sub
                }
                _ => break,
            };
            let result = self.parse_term();
            ret = ret.and_then(|ret| result.map(|num| op(ret, num)));
        }
        ret
    }

    fn parse_term(&mut self) -> Result<f64, ParseFloatError> {
        let mut ret = self.parse_num();
        loop {
            let op = match self.next() {
                Some('×') => {
                    self.skip();
                    mul
                },
                Some('÷') => {
                    self.skip();
                    div
                }
                _ => break,
            };
            let result = self.parse_num();
            ret = ret.and_then(|ret| result.map(|num| op(ret, num)));
        }
        ret
    }

    fn parse_num(&mut self) -> Result<f64, ParseFloatError> {
        let mut num = String::new();
        loop {
            match self.iter.peek().cloned() {
                Some('+') | Some('×') | Some('÷') | Some(')') | None => break,
                Some('-') if !num.is_empty() => break,
                Some('(') => {
                    self.skip();
                    let ret = self.parse_expression();
                    self.skip();
                    return ret;
                }
                Some(d) => num.push(d)
            }
            self.iter.next();
        }
        num.parse()
    }

    fn next(&mut self) -> Option<char> {
        self.iter.peek().cloned()
    }

    fn skip(&mut self) {
        self.iter.next();
    }
}

fn add(acc: f64, num: f64) -> f64 {
    acc + num
}

fn sub(acc: f64, num: f64) -> f64 {
    acc - num
}

fn mul(acc: f64, num: f64) -> f64 {
    acc * num
}

fn div(acc: f64, num: f64) -> f64 {
    acc / num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluate_negative_number() {
        let calc = Calculator::new(Cow::Borrowed("-12"));
        assert_eq!(calc.calculate(), Ok(-12.0));
    }

    #[test]
    fn evaluate_addition() {
        let calc = Calculator::new(Cow::Borrowed("91+2"));
        assert_eq!(calc.calculate(), Ok(93.0));
    }

    #[test]
    fn evaluate_subtraction() {
        let calc = Calculator::new(Cow::Borrowed("12-5"));
        assert_eq!(calc.calculate(), Ok(7.0));
    }

    #[test]
    fn evaluate_multiplication() {
        let calc = Calculator::new(Cow::Borrowed("23×1"));
        assert_eq!(calc.calculate(), Ok(23.0));
    }

    #[test]
    fn evaluate_division() {
        let calc = Calculator::new(Cow::Borrowed("33÷3"));
        assert_eq!(calc.calculate(), Ok(11.0));
    }

    #[test]
    fn evaluate_multiple_operations() {
        let calc = Calculator::new(Cow::Borrowed("4×2-33÷3+5"));
        assert_eq!(calc.calculate(), Ok(2.0));
    }

    #[test]
    fn evaluate_operations_with_parenthesis() {
        let calc = Calculator::new(Cow::Borrowed("4×(2-40÷(3+5))"));
        assert_eq!(calc.calculate(), Ok(-12.0));
    }
}
