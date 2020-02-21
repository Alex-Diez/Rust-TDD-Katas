use std::borrow::Cow;
use std::iter::Peekable;
use std::num::ParseFloatError;
use std::str::Chars;

type Arg = Result<f64, ParseFloatError>;

#[derive(PartialEq, Debug)]
pub enum Ast {
    Num(Arg),
    Operand(char, Box<Ast>, Box<Ast>)
}

impl Ast {
    pub fn num(arg: Arg) -> Ast {
        Ast::Num(arg)
    }

    pub fn operand(sign: char, left: Ast, right: Ast) -> Ast {
        Ast::Operand(sign, Box::new(left), Box::new(right))
    }
}

pub fn parse_expression<'s>(src: Cow<'s, str>) -> Ast {
    let mut iter = src.chars().peekable();
    parse_subexpression(iter.by_ref())
}

fn parse_subexpression(iter: &mut Peekable<Chars>) -> Ast {
    let mut root = parse_term(iter.by_ref());
    while let Some(operand) = low_priority_operand(iter.by_ref()) {
        root = Ast::operand(operand, root, parse_term(iter.by_ref()));
    }
    root
}

fn low_priority_operand(iter: &mut Peekable<Chars>) -> Option<char> {
    match iter.peek() {
        Some(&'+') => {
            iter.next()
        },
        Some(&'-') => {
            iter.next()
        }
        _ => None
    }
}

fn parse_term(iter: &mut Peekable<Chars>) -> Ast {
    let mut root = parse_num(iter.by_ref());
    while let Some(operand) = high_priority_operand(iter.by_ref()) {
        root = Ast::operand(operand, root, parse_num(iter.by_ref()));
    }
    root
}

fn high_priority_operand(iter: &mut Peekable<Chars>) -> Option<char> {
    match iter.peek() {
        Some(&'×') => {
            iter.next()
        },
        Some(&'÷') => {
            iter.next()
        }
        _ => None
    }
}

fn parse_num(iter: &mut Peekable<Chars>) -> Ast {
    let mut num = String::new();
    loop {
        match iter.peek().cloned() {
            Some('+') | Some('×') | Some('÷') | Some(')') | None => break,
            Some('-') if !num.is_empty() => break,
            Some('(') => {
                iter.next();
                let sub_root = parse_subexpression(iter.by_ref());
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

    fn num(num: f64) -> Ast {
        Ast::Num(Ok(num))
    }

    #[test]
    fn parse_negative_number() {
        assert_eq!(parse_expression(Cow::Borrowed("-4")), num(-4.0));
    }

    #[test]
    fn parse_addition() {
        assert_eq!(parse_expression(Cow::Borrowed("3+2")), Ast::operand('+', num(3.0), num(2.0)));
    }

    #[test]
    fn parse_subtraction() {
        assert_eq!(parse_expression(Cow::Borrowed("5-2")), Ast::operand('-', num(5.0), num(2.0)));
    }

    #[test]
    fn parse_multiplication() {
        assert_eq!(parse_expression(Cow::Borrowed("3×4")), Ast::operand('×', num(3.0), num(4.0)));
    }

    #[test]
    fn parse_division() {
        assert_eq!(parse_expression(Cow::Borrowed("5÷2")), Ast::operand('÷', num(5.0), num(2.0)));
    }

    #[test]
    fn parse_many_operations() {
        assert_eq!(
            parse_expression(Cow::Borrowed("3-4÷2+1×5")),
            Ast::operand(
                '+',
                Ast::operand(
                    '-', num(3.0),
                    Ast::operand(
                        '÷',
                        num(4.0),
                        num(2.0)
                    )
                ),
                Ast::operand(
                    '×',
                    num(1.0),
                    num(5.0)
                )
            )
        );
    }

    #[test]
    fn parse_expression_with_parenthesis() {
        assert_eq!(
            parse_expression(Cow::Borrowed("(3-4)÷(2+1)×5")),
            Ast::operand(
                '×',
                Ast::operand(
                    '÷',
                    Ast::operand(
                        '-',
                        num(3.0),
                        num(4.0)
                    ),
                    Ast::operand(
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
