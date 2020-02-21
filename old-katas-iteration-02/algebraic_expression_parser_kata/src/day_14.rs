use std::borrow::Cow;
use std::iter::Peekable;
use std::num::ParseFloatError;
use std::str::Chars;

#[derive(PartialEq, Debug)]
pub enum Ast {
    Num(Result<f64, ParseFloatError>),
    Operation(char, Box<Ast>, Box<Ast>),
}

pub struct Parser<'p> {
    iter: Peekable<Chars<'p>>
}

impl<'p> Parser<'p> {
    pub fn new<'s>(src: &'p Cow<'s, str>) -> Self {
        Parser { iter: src.chars().peekable() }
    }

    pub fn parse_expression(&mut self) -> Ast {
        let mut root = self.parse_term();
        while let Some(operand) = self.low_priority_operand() {
            root = Ast::Operation(operand, Box::new(root), Box::new(self.parse_term()))
        }
        root
    }

    fn low_priority_operand(&mut self) -> Option<char> {
        match self.iter.peek() {
            Some(&'+') | Some(&'-') => self.iter.next(),
            _ => None
        }
    }

    fn parse_term(&mut self) -> Ast {
        let mut root = self.parse_num();
        while let Some(operand) = self.high_priority_operand() {
            root = Ast::Operation(operand, Box::new(root), Box::new(self.parse_num()))
        }
        root
    }

    fn high_priority_operand(&mut self) -> Option<char> {
        match self.iter.peek() {
            Some(&'×') | Some(&'÷') => self.iter.next(),
            _ => None
        }
    }

    fn parse_num(&mut self) -> Ast {
        let mut num = String::new();
        while let Some(character) = self.iter.peek().cloned() {
            match character {
                '+' | '×' | '÷' | ')' => break,
                '-' if !num.is_empty() => break,
                '(' => {
                    self.iter.next();
                    let sub_root = self.parse_expression();
                    self.iter.next();
                    return sub_root;
                }
                digit => num.push(digit)
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

    fn operation(operand: char, left: Ast, right: Ast) -> Ast {
        Ast::Operation(operand, Box::new(left), Box::new(right))
    }

    #[test]
    fn parse_negative_number() {
        let expr = Cow::Borrowed("-4");
        assert_eq!(Parser::new(&expr).parse_expression(), num(-4.0));
    }

    #[test]
    fn parse_addition() {
        let expr = Cow::Borrowed("3+2");
        assert_eq!(Parser::new(&expr).parse_expression(), operation('+', num(3.0), num(2.0)));
    }

    #[test]
    fn parse_subtraction() {
        let expr = Cow::Borrowed("3-1");
        assert_eq!(Parser::new(&expr).parse_expression(), operation('-', num(3.0), num(1.0)));
    }

    #[test]
    fn parse_multiplication() {
        let expr = Cow::Borrowed("3×4");
        assert_eq!(Parser::new(&expr).parse_expression(), operation('×', num(3.0), num(4.0)));
    }

    #[test]
    fn parse_division() {
        let expr = Cow::Borrowed("4÷2");
        assert_eq!(Parser::new(&expr).parse_expression(), operation('÷', num(4.0), num(2.0)));
    }

    #[test]
    fn parse_many_operations() {
        let expr = Cow::Borrowed("3÷1+2×3÷3-1");
        assert_eq!(
            Parser::new(&expr).parse_expression(),
            operation(
                '-',
                operation(
                    '+',
                    operation(
                        '÷',
                        num(3.0),
                        num(1.0)
                    ),
                    operation(
                        '÷',
                        operation(
                            '×',
                            num(2.0),
                            num(3.0)
                        ),
                        num(3.0)
                    )
                ),
                num(1.0)
            )
        );
    }

    #[test]
    fn parse_expression_with_parenthesis() {
        let expr = Cow::Borrowed("3÷(1+2)×(3-1)");
        assert_eq!(
            Parser::new(&expr).parse_expression(),
            operation(
                '×',
                operation(
                    '÷',
                    num(3.0),
                    operation(
                        '+',
                        num(1.0),
                        num(2.0)
                    )
                ),
                operation(
                    '-',
                    num(3.0),
                    num(1.0)
                )
            )
        );
    }
}
