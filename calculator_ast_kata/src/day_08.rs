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
        num.as_str()
            .parse()
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
        while let Some(op) = Self::parse_high_priority_op(chars.by_ref()) {
            let right = Ast::parse_num(chars.by_ref());
            root = match (root, right) {
                (Ok(left), Ok(right)) => Ok(Ast::Op(op, Box::new(left), Box::new(right))),
                _ => Err(ParseAstError),
            }
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
        while let Some(op) = Self::parse_low_priority_op(chars.by_ref()) {
            let right = Ast::parse_term(chars.by_ref());
            root = match (root, right) {
                (Ok(left), Ok(right)) => Ok(Ast::Op(op, Box::new(left), Box::new(right))),
                _ => Err(ParseAstError),
            }
        }
        root
    }
}

impl FromStr for Ast {
    type Err = ParseAstError;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        let mut chars = source.chars().peekable();
        Self::parse_expression(chars.by_ref())
    }
}

#[derive(Debug, PartialEq)]
pub struct ParseAstError;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error() {
        assert_eq!(Ast::from_str("abc"), Err(ParseAstError));
    }

    #[test]
    fn number() {
        assert_eq!("5".parse(), Ok(Ast::Num(5.0)));
    }

    #[test]
    fn negative_number() {
        assert_eq!("-9".parse(), Ok(Ast::Num(-9.0)))
    }

    #[test]
    fn addition() {
        assert_eq!(
            "4+3".parse(),
            Ok(Ast::Op(
                '+',
                Box::new(Ast::Num(4.0)),
                Box::new(Ast::Num(3.0)),
            ))
        )
    }

    #[test]
    fn left_hand_error_low_priority_op() {
        assert_eq!(Ast::from_str("abc+3"), Err(ParseAstError))
    }

    #[test]
    fn right_hand_error_low_priority_op() {
        assert_eq!(Ast::from_str("4+abc"), Err(ParseAstError))
    }

    #[test]
    fn subtraction() {
        assert_eq!(
            "4-3".parse(),
            Ok(Ast::Op(
                '-',
                Box::new(Ast::Num(4.0)),
                Box::new(Ast::Num(3.0)),
            ))
        )
    }

    #[test]
    fn multiplication() {
        assert_eq!(
            "5*8".parse(),
            Ok(Ast::Op(
                '*',
                Box::new(Ast::Num(5.0)),
                Box::new(Ast::Num(8.0))
            ))
        )
    }

    #[test]
    fn division() {
        assert_eq!(
            "6/3".parse(),
            Ok(Ast::Op(
                '/',
                Box::new(Ast::Num(6.0)),
                Box::new(Ast::Num(3.0))
            ))
        )
    }

    #[test]
    fn many_operations() {
        assert_eq!(
            "5+9*3-27/3".parse(),
            Ok(Ast::Op(
                '-',
                Box::new(Ast::Op(
                    '+',
                    Box::new(Ast::Num(5.0)),
                    Box::new(Ast::Op(
                        '*',
                        Box::new(Ast::Num(9.0)),
                        Box::new(Ast::Num(3.0))
                    ))
                )),
                Box::new(Ast::Op(
                    '/',
                    Box::new(Ast::Num(27.0)),
                    Box::new(Ast::Num(3.0))
                ))
            ))
        )
    }

    #[test]
    fn left_hand_error_high_priority_op() {
        assert_eq!(Ast::from_str("abc*3"), Err(ParseAstError))
    }

    #[test]
    fn right_hand_error_high_priority_op() {
        assert_eq!(Ast::from_str("4*abc"), Err(ParseAstError))
    }
}
