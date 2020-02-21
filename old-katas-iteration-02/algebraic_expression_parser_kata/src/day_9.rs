use std::borrow::Cow;
use std::iter::Peekable;
use std::num::ParseFloatError;
use std::str::Chars;

#[derive(PartialEq, Debug)]
pub enum Ast {
    Num(Result<f64, ParseFloatError>),
    Operand(char, Box<Ast>, Box<Ast>)
}

pub struct Parser<'s> {
    iter: Peekable<Chars<'s>>
}

impl <'s> Parser<'s> {
    pub fn new(iter: Peekable<Chars<'s>>) -> Self {
        Parser { iter }
    }

    pub fn parse_expression(&mut self) -> Ast {
        let mut root = self.parse_term();
        while let Some(operand) = self.low_priority_operand() {
            root = Ast::Operand(operand, Box::new(root), Box::new(self.parse_term()))
        }
        root
    }

    fn low_priority_operand(&mut self) -> Option<char> {
        match self.iter.peek() {
            Some(&'+') | Some(&'-') => self.iter.next(),
            _ => None
        }
    }

    pub fn parse_term(&mut self) -> Ast {
        let mut root = self.parse_num();
        while let Some(operand) = self.high_priority_operand() {
            root = Ast::Operand(operand, Box::new(root), Box::new(self.parse_num()))
        }
        root
    }

    fn high_priority_operand(&mut self) -> Option<char> {
        match self.iter.peek() {
            Some(&'×') => {
                self.iter.next()
            }
            Some(&'÷') => {
                self.iter.next()
            }
            _ => None
        }
    }

    pub fn parse_num(&mut self) -> Ast {
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

pub fn parse_expression<'s>(src: Cow<'s, str>) -> Ast {
    Parser::new(src.chars().peekable()).parse_expression()
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
        assert_eq!(parse_expression(Cow::Borrowed("-2")), num(-2.0));
    }

    #[test]
    fn parse_addition() {
        assert_eq!(parse_expression(Cow::Borrowed("4+2")), operand('+', num(4.0), num(2.0)));
    }

    #[test]
    fn parse_subtraction() {
        assert_eq!(parse_expression(Cow::Borrowed("5-1")), operand('-', num(5.0), num(1.0)));
    }

    #[test]
    fn parse_multiplication() {
        assert_eq!(parse_expression(Cow::Borrowed("4×2")), operand('×', num(4.0), num(2.0)));
    }

    #[test]
    fn parse_division() {
        assert_eq!(parse_expression(Cow::Borrowed("5÷1")), operand('÷', num(5.0), num(1.0)));
    }

    #[test]
    fn parse_many_operations() {
        assert_eq!(
            parse_expression(Cow::Borrowed("4-2×2+1÷5")),
            operand(
                '+',
                operand(
                    '-',
                    num(4.0),
                    operand(
                        '×',
                        num(2.0),
                        num(2.0)
                    )
                ),
                operand(
                    '÷',
                    num(1.0),
                    num(5.0)
                )
            )
        );
    }

    #[test]
    fn parse_operations_with_parenthesis() {
        assert_eq!(
            parse_expression(Cow::Borrowed("(4-2)×(2+1)÷5")),
            operand(
                '÷',
                operand(
                    '×',
                    operand(
                        '-',
                        num(4.0),
                        num(2.0)
                    ),
                    operand(
                        '+',
                        num(2.0),
                        num(1.0)
                    )
                ),
                num(5.0)
            )
        );
    }
}
