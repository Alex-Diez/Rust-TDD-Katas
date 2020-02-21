use std::borrow::Cow;
use std::iter::Peekable;
use std::str::{Chars, FromStr};

#[derive(PartialEq, Debug)]
pub enum Ast<N: FromStr> {
    Num(Result<N, N::Err>),
    Operand(char, Box<Ast<N>>, Box<Ast<N>>)
}

pub fn parse_expression<'s, F: FromStr>(src: Cow<'s, str>) -> Ast<F> {
    let mut iter = src.chars().peekable();
    parse_sub_expression(iter.by_ref())
}

fn parse_sub_expression<F: FromStr>(iter: &mut Peekable<Chars>) -> Ast<F> {
    let mut root = parse_term(iter.by_ref());
    while let Some(operand) = low_priority_operand(iter.by_ref()) {
        root = Ast::Operand(operand, Box::new(root), Box::new(parse_term(iter.by_ref())))
    }
    root
}

fn low_priority_operand(iter: &mut Peekable<Chars>) -> Option<char> {
    match iter.peek() {
        Some(&'+') | Some(&'-') => iter.next(),
        _ => None
    }
}

fn parse_term<F: FromStr>(iter: &mut Peekable<Chars>) -> Ast<F> {
    let mut root = parse_num(iter.by_ref());
    while let Some(operand) = high_priority_operand(iter.by_ref()) {
        root = Ast::Operand(operand, Box::new(root), Box::new(parse_num(iter.by_ref())))
    }
    root
}

fn high_priority_operand(iter: &mut Peekable<Chars>) -> Option<char> {
    match iter.peek() {
        Some(&'×') | Some(&'÷') => iter.next(),
        _ => None
    }
}

fn parse_num<F: FromStr>(iter: &mut Peekable<Chars>) -> Ast<F> {
    let mut num = String::new();
    loop {
        match iter.peek().cloned() {
            Some('+') | Some('×') | Some('÷') | Some(')') | None => break,
            Some('-') if !num.is_empty() => break,
            Some('(') => {
                iter.next();
                let sub_root = parse_sub_expression(iter.by_ref());
                iter.next();
                return sub_root;
            }
            Some(d) => num.push(d)
        }
        iter.next();
    }
    Ast::Num(num.parse())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn num(num: f64) -> Ast<f64> {
        Ast::Num(Ok(num))
    }

    fn operand(sign: char, left: Ast<f64>, right: Ast<f64>) -> Ast<f64> {
        Ast::Operand(sign, Box::new(left), Box::new(right))
    }

    #[test]
    fn parse_negative_number() {
        assert_eq!(parse_expression(Cow::Borrowed("-3")), num(-3.0));
    }

    #[test]
    fn parse_addition() {
        assert_eq!(parse_expression(Cow::Borrowed("5+2")), operand('+', num(5.0), num(2.0)));
    }

    #[test]
    fn parse_subtraction() {
        assert_eq!(parse_expression(Cow::Borrowed("3-4")), operand('-', num(3.0), num(4.0)));
    }

    #[test]
    fn parse_multiplication() {
        assert_eq!(parse_expression(Cow::Borrowed("7×1")), operand('×', num(7.0), num(1.0)));
    }

    #[test]
    fn parse_division() {
        assert_eq!(parse_expression(Cow::Borrowed("5÷2")), operand('÷', num(5.0), num(2.0)));
    }

    #[test]
    fn parse_many_operations() {
        assert_eq!(
            parse_expression(
                Cow::Borrowed("4-2÷3+2×1")
            ),
            operand(
                '+',
                operand(
                    '-',
                    num(4.0),
                    operand(
                        '÷',
                        num(2.0),
                        num(3.0)
                    )
                ),
                operand(
                    '×',
                    num(2.0),
                    num(1.0)
                )
            )
        );
    }

    #[test]
    fn parse_operations_with_parenthesis() {
        assert_eq!(
            parse_expression(
                Cow::Borrowed("(4-2)÷(3+2)×1")
            ),
            operand(
                '×',
                operand(
                    '÷',
                    operand(
                        '-',
                        num(4.0),
                        num(2.0)
                    ),
                    operand(
                        '+',
                        num(3.0),
                        num(2.0)
                    )
                ),
                num(1.0)
            )
        );
    }
}
