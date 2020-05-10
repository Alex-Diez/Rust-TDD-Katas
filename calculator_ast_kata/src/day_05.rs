use std::iter::Peekable;
use std::str::{Chars, FromStr};

#[derive(Debug, PartialEq)]
pub enum Ast {
    Num(f64),
    Op(char, Box<Ast>, Box<Ast>),
}

impl Ast {
    fn parse_num(chars: &mut Peekable<Chars>) -> Result<Self, ParseAstError> {
        let mut num = String::new();
        while let Some(&char) = chars.peek() {
            match char {
                '+' | '*' | '/' => break,
                '-' if !num.is_empty() => break,
                _ => {
                    chars.next();
                    num.push(char);
                }
            }
        }
        f64::from_str(num.as_str())
            .map(Ast::Num)
            .map_err(|_| ParseAstError)
    }

    fn parse_high_priority_op(chars: &mut Peekable<Chars>) -> Option<char> {
        match chars.peek() {
            Some(&'*') | Some(&'/') => chars.next(),
            _ => None,
        }
    }

    fn parse_term(chars: &mut Peekable<Chars>) -> Result<Self, ParseAstError> {
        let mut root = Ast::parse_num(chars.by_ref());
        while let Some(op) = Ast::parse_high_priority_op(chars.by_ref()) {
            root = Ok(Ast::Op(
                op,
                Box::new(root.unwrap()),
                Box::new(Ast::parse_num(chars.by_ref()).unwrap()),
            ))
        }
        root
    }

    fn parse_low_priority_op(chars: &mut Peekable<Chars>) -> Option<char> {
        match chars.peek() {
            Some(&'+') | Some(&'-') => chars.next(),
            _ => None,
        }
    }

    fn parse_expression(chars: &mut Peekable<Chars>) -> Result<Self, ParseAstError> {
        let mut root = Ast::parse_term(chars.by_ref());
        while let Some(op) = Ast::parse_low_priority_op(chars.by_ref()) {
            root = Ok(Ast::Op(
                op,
                Box::new(root.unwrap()),
                Box::new(Ast::parse_term(chars.by_ref()).unwrap()),
            ))
        }
        root
    }
}

impl FromStr for Ast {
    type Err = ParseAstError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars().peekable();
        Ast::parse_expression(chars.by_ref())
    }
}

#[derive(Debug, PartialEq)]
pub struct ParseAstError;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error() {
        assert_eq!(Ast::from_str("abc"), Err(ParseAstError))
    }

    #[test]
    fn number() {
        assert_eq!(Ast::from_str("1"), Ok(Ast::Num(1.0)))
    }

    #[test]
    fn negative_number() {
        assert_eq!(Ast::from_str("-3"), Ok(Ast::Num(-3.0)))
    }

    #[test]
    fn addition() {
        assert_eq!(
            Ast::from_str("3+5"),
            Ok(Ast::Op(
                '+',
                Box::new(Ast::Num(3.0)),
                Box::new(Ast::Num(5.0))
            ))
        )
    }

    #[test]
    fn subtraction() {
        assert_eq!(
            Ast::from_str("4-9"),
            Ok(Ast::Op(
                '-',
                Box::new(Ast::Num(4.0)),
                Box::new(Ast::Num(9.0))
            ))
        )
    }

    #[test]
    fn multiplication() {
        assert_eq!(
            Ast::from_str("5*9"),
            Ok(Ast::Op(
                '*',
                Box::new(Ast::Num(5.0)),
                Box::new(Ast::Num(9.0))
            ))
        )
    }

    #[test]
    fn division() {
        assert_eq!(
            Ast::from_str("33/2"),
            Ok(Ast::Op(
                '/',
                Box::new(Ast::Num(33.0)),
                Box::new(Ast::Num(2.0))
            ))
        )
    }

    #[test]
    fn many_operations() {
        assert_eq!(
            Ast::from_str("5+4*2-9/3"),
            Ok(Ast::Op(
                '-',
                Box::new(Ast::Op(
                    '+',
                    Box::new(Ast::Num(5.0)),
                    Box::new(Ast::Op(
                        '*',
                        Box::new(Ast::Num(4.0)),
                        Box::new(Ast::Num(2.0))
                    ))
                )),
                Box::new(Ast::Op(
                    '/',
                    Box::new(Ast::Num(9.0)),
                    Box::new(Ast::Num(3.0))
                ))
            ))
        )
    }
}
