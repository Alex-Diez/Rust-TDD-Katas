use std::borrow::Cow;
use std::iter::Peekable;
use std::num::ParseFloatError;
use std::str::Chars;

#[derive(PartialEq, Debug)]
pub enum Ast {
    Num(Result<f64, ParseFloatError>),
    Operation(char, Box<Ast>, Box<Ast>),
}

impl Ast {
    pub fn num(num: Result<f64, ParseFloatError>) -> Self {
        Ast::Num(num)
    }

    pub fn operation(sign: char, left: Ast, right: Ast) -> Self {
        Ast::Operation(sign, Box::new(left), Box::new(right))
    }
}

pub fn parse_expression<'s>(src: Cow<'s, str>) -> Ast {
    let mut iter = src.chars().peekable();
    let mut left = parse_term(iter.by_ref());
    loop {
        match iter.peek().cloned() {
            Some('+') => {
                iter.next();
                let right = parse_term(iter.by_ref());
                left = Ast::operation('+', left, right)
            }
            Some('-') => {
                iter.next();
                let right = parse_term(iter.by_ref());
                left = Ast::operation('-', left, right)
            }
            _ => break
        }
    }
    left
}

fn parse_term(iter: &mut Peekable<Chars>) -> Ast {
    let mut left = parse_num(iter.by_ref());
    loop {
        match iter.peek().cloned() {
            Some('×') => {
                iter.next();
                let right = parse_num(iter.by_ref());
                left = Ast::operation('×', left, right)
            }
            Some('÷') => {
                iter.next();
                let right = parse_num(iter.by_ref());
                left = Ast::operation('÷', left, right)
            }
            _ => break
        }
    }
    left
}

fn parse_num(iter: &mut Peekable<Chars>) -> Ast {
    let mut num = String::new();
    loop {
        match iter.peek().cloned() {
            Some('+') | Some('×') | Some('÷') | None => break,
            Some('-') if !num.is_empty() => break,
            Some(d) => num.push(d)
        }
        iter.next();
    }
    Ast::Num(num.parse())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_negative_number() {
        assert_eq!(parse_expression(Cow::Borrowed("-9")), ok_num(-9.0));
    }

    #[test]
    fn parse_addition() {
        assert_eq!(parse_expression(Cow::Borrowed("3+4")), ok_operation('+', 3.0, 4.0));
    }

    #[test]
    fn parse_subtraction() {
        assert_eq!(parse_expression(Cow::Borrowed("6-2")), ok_operation('-', 6.0, 2.0));
    }

    #[test]
    fn parse_multiplication() {
        assert_eq!(parse_expression(Cow::Borrowed("3×2")), ok_operation('×', 3.0, 2.0));
    }

    #[test]
    fn parse_division() {
        assert_eq!(parse_expression(Cow::Borrowed("12÷3")), ok_operation('÷', 12.0, 3.0));
    }

    #[test]
    fn parse_many_operations() {
        assert_eq!(
            parse_expression(Cow::Borrowed("3+2÷1-3×2")),
            Ast::operation(
                '-',
                Ast::operation(
                    '+',
                    ok_num(3.0),
                    ok_operation('÷', 2.0, 1.0)
                ),
                ok_operation('×', 3.0, 2.0)
            )
        );
    }

    fn ok_num(num: f64) -> Ast {
        Ast::Num(Ok(num))
    }

    fn ok_operation(sign: char, left: f64, right: f64) -> Ast {
        Ast::operation(sign, ok_num(left), ok_num(right))
    }
}
