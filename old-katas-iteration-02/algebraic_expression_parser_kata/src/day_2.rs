use std::borrow::Cow;
use std::iter::Peekable;
use std::num::ParseFloatError;
use std::str::Chars;

type Number = Result<f64, ParseFloatError>;

#[derive(PartialEq, Debug)]
pub enum Ast {
    Num(Number),
    Operation(char, Box<Ast>, Box<Ast>),
}

impl Ast {
    pub fn num(num: Number) -> Self {
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
                left = Ast::operation('+', left, parse_term(iter.by_ref()))
            }
            Some('-') => {
                iter.next();
                left = Ast::operation('-', left, parse_term(iter.by_ref()))
            }
            _ => break
        }
    }
    left
}

fn parse_term(iter: &mut Peekable<Chars>) -> Ast {
    let mut left = Ast::Num(parse_num(iter.by_ref()));
    loop {
        match iter.peek().cloned() {
            Some('×') => {
                iter.next();
                left = Ast::operation('×', left, Ast::Num(parse_num(iter.by_ref())))
            }
            Some('÷') => {
                iter.next();
                left = Ast::operation('÷', left, Ast::Num(parse_num(iter.by_ref())))
            }
            _ => break
        }
    }
    left
}

fn parse_num(iter: &mut Peekable<Chars>) -> Number {
    let mut num = String::new();
    loop {
        match iter.peek().cloned() {
            Some('+') | Some('×') | Some('÷') | None => break,
            Some('-') if !num.is_empty() => break,
            Some(d) => num.push(d)
        }
        iter.next();
    }
    num.parse()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_negative_number() {
        assert_eq!(parse_expression(Cow::Borrowed("-2")), Ast::Num(Ok(-2.0)));
    }

    #[test]
    fn parse_addition() {
        assert_eq!(parse_expression(Cow::Borrowed("3+2")), Ast::operation('+', Ast::num(Ok(3.0)), Ast::num(Ok(2.0))));
    }

    #[test]
    fn parse_subtraction() {
        assert_eq!(parse_expression(Cow::Borrowed("4-2")), Ast::operation('-', Ast::num(Ok(4.0)), Ast::num(Ok(2.0))));
    }

    #[test]
    fn parse_multiplication() {
        assert_eq!(parse_expression(Cow::Borrowed("6×3")), Ast::operation('×', Ast::num(Ok(6.0)), Ast::num(Ok(3.0))));
    }

    #[test]
    fn parse_division() {
        assert_eq!(parse_expression(Cow::Borrowed("10÷2")), Ast::operation('÷', Ast::num(Ok(10.0)), Ast::num(Ok(2.0))));
    }

    #[test]
    fn parse_many_operations() {
        assert_eq!(
            parse_expression(Cow::Borrowed("4+1×3-10÷2")),
            Ast::operation(
                '-',
                Ast::operation(
                    '+',
                    Ast::num(Ok(4.0)),
                    Ast::operation('×', Ast::num(Ok(1.0)), Ast::num(Ok(3.0))),
                ),
                Ast::operation('÷', Ast::num(Ok(10.0)), Ast::num(Ok(2.0))),
            )
        )
    }
}
