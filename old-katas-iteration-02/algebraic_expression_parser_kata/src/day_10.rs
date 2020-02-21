use std::borrow::Cow;
use std::iter::Peekable;
use std::num::ParseFloatError;
use std::str::Chars;

#[derive(PartialEq, Debug)]
pub enum Ast {
    Num(Result<f64, ParseFloatError>),
    Operand(char, Box<Ast>, Box<Ast>),
}

pub struct Parser<'p> {
    iter: Peekable<Chars<'p>>
}

impl<'p> Parser<'p> {
    pub fn create_parser<'s>(src: &'p Cow<'s, str>) -> Self {
        Parser { iter: src.chars().peekable() }
    }

    pub fn parse_expression(&mut self) -> Ast {
        let mut left = self.parse_term();
        while let Some(operand) = self.low_priority_operand() {
            left = Ast::Operand(operand, Box::new(left), Box::new(self.parse_term()))
        }
        left
    }

    fn low_priority_operand(&mut self) -> Option<char> {
        match self.iter.peek() {
            Some(&'+') | Some(&'-') => self.iter.next(),
            _ => None
        }
    }

    fn parse_term(&mut self) -> Ast {
        let mut left = self.parse_num();
        while let Some(operand) = self.high_priority_operand() {
            left = Ast::Operand(operand, Box::new(left), Box::new(self.parse_num()))
        }
        left
    }

    fn high_priority_operand(&mut self) -> Option<char> {
        match self.iter.peek() {
            Some(&'×') | Some(&'÷') => self.iter.next(),
            _ => None
        }
    }

    fn parse_num(&mut self) -> Ast {
        let mut num = String::new();
        loop {
            match self.iter.peek().cloned() {
                Some('+') | Some('×') | Some('÷') | Some(')') | None => break,
                Some('-') if !num.is_empty() => break,
                Some('(') => {
                    self.iter.next();
                    let sub_root = self.parse_expression();
                    self.iter.next();
                    return sub_root;
                }
                Some(d) => num.push(d)
            }
            self.iter.next();
        }
        Ast::Num(num.parse())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn num(num: f64) -> Ast {
        Ast::Num(Ok(num))
    }

    fn operand(sign: char, left: Ast, right: Ast) -> Ast {
        Ast::Operand(sign, Box::new(left), Box::new(right))
    }

    #[test]
    fn parse_negative_number() {
        let expr = Cow::Borrowed("-5");
        assert_eq!(Parser::create_parser(&expr).parse_expression(), num(-5.0));
    }

    #[test]
    fn parse_addition() {
        let expr = Cow::Borrowed("4+2");
        assert_eq!(Parser::create_parser(&expr).parse_expression(), operand('+', num(4.0), num(2.0)));
    }

    #[test]
    fn parse_subtraction() {
        let expr = Cow::Borrowed("3-1");
        assert_eq!(Parser::create_parser(&expr).parse_expression(), operand('-', num(3.0), num(1.0)));
    }

    #[test]
    fn parse_multiplication() {
        let expr = Cow::Borrowed("5×2");
        assert_eq!(Parser::create_parser(&expr).parse_expression(), operand('×', num(5.0), num(2.0)));
    }

    #[test]
    fn parse_division() {
        let expr = Cow::Borrowed("4÷2");
        assert_eq!(Parser::create_parser(&expr).parse_expression(), operand('÷', num(4.0), num(2.0)));
    }

    #[test]
    fn parse_many_operations() {
        let expr = Cow::Borrowed("4+2÷3-1×2");
        assert_eq!(
            Parser::create_parser(&expr).parse_expression(),
            operand(
                '-',
                operand(
                    '+',
                    num(4.0),
                    operand(
                        '÷',
                        num(2.0),
                        num(3.0)
                    )
                ),
                operand(
                    '×',
                    num(1.0),
                    num(2.0)
                )
            )
        );
    }

    #[test]
    fn parse_expression_with_parenthesis() {
        let expr = Cow::Borrowed("(4+2)÷(3-1)×2");
        assert_eq!(
            Parser::create_parser(&expr).parse_expression(),
            operand(
                '×',
                operand(
                    '÷',
                    operand(
                        '+',
                        num(4.0),
                        num(2.0)
                    ),
                    operand(
                        '-',
                        num(3.0),
                        num(1.0)
                    )
                ),
                num(2.0)
            )
        );
    }
}
