use std::borrow::Cow;
use std::iter::Peekable;
use std::num::ParseFloatError;
use std::str::Chars;

#[derive(PartialEq, Debug)]
pub enum Ast {
    Num(Result<f64, ParseFloatError>),
    Operand(char, Box<Ast>, Box<Ast>)
}

pub fn parse_expression<'s>(src: Cow<'s, str>) -> Ast {
    let mut iter = src.chars().peekable();
    parse_sub_expression(iter.by_ref())
}

fn parse_sub_expression(iter: &mut Peekable<Chars>) -> Ast {
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

fn parse_term(iter: &mut Peekable<Chars>) -> Ast {
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

fn parse_num(iter: &mut Peekable<Chars>) -> Ast {
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

    fn num(num: f64) -> Ast {
        Ast::Num(Ok(num))
    }

    fn operand(sign: char, left: Ast, right: Ast) -> Ast {
        Ast::Operand(sign, Box::new(left), Box::new(right))
    }

    #[test]
    fn parse_negative_number() {
        assert_eq!(parse_expression(Cow::Borrowed("-3")), num(-3.0));
    }

    #[test]
    fn parse_addition() {
        assert_eq!(parse_expression(Cow::Borrowed("4+2")), operand('+', num(4.0), num(2.0)));
    }

    #[test]
    fn parse_subtraction() {
        assert_eq!(parse_expression(Cow::Borrowed("4-2")), operand('-', num(4.0), num(2.0)));
    }

    #[test]
    fn parse_multiplication() {
        assert_eq!(parse_expression(Cow::Borrowed("5×4")), operand('×', num(5.0), num(4.0)));
    }

    #[test]
    fn parse_division() {
        assert_eq!(parse_expression(Cow::Borrowed("6÷2")), operand('÷', num(6.0), num(2.0)));
    }

    #[test]
    fn parse_many_operations() {
        assert_eq!(
            parse_expression(Cow::Borrowed("4+2÷1-5×2")),
            operand(
                '-',
                operand(
                    '+',
                    num(4.0),
                    operand(
                        '÷',
                        num(2.0),
                        num(1.0)
                    )
                ),
                operand(
                    '×',
                    num(5.0),
                    num(2.0)
                )
            )
        );
    }

    #[test]
    fn parse_operations_with_parenthesis() {
        assert_eq!(
            parse_expression(Cow::Borrowed("(4+2)÷(1-5)×2")),
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
                        num(1.0),
                        num(5.0)
                    )
                ),
                num(2.0)
            )
        );
    }
}
