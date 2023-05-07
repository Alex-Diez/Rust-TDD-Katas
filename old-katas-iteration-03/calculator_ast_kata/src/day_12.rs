use std::iter::Peekable;
use std::str::{Chars, FromStr};

type Cursor<'c> = Peekable<Chars<'c>>;

#[derive(Debug, PartialEq)]
pub enum Ast {
    Num(f64),
    Op(char, Box<Ast>, Box<Ast>),
}

impl Ast {
    fn parse_num(cursor: &mut Cursor) -> Result<Self, ParseAstError> {
        let mut num = String::new();
        while let Some(&char) = cursor.peek() {
            match char {
                '+' => break,
                '-' if !num.is_empty() => break,
                _ => {
                    num.push(char);
                    cursor.next();
                }
            }
        }
        num.parse().map(Ast::Num).map_err(|_| ParseAstError(0))
    }
}

impl FromStr for Ast {
    type Err = ParseAstError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars().peekable();
        let left = Self::parse_num(chars.by_ref());
        if let Some(op) = chars.next() {
            match (left, Self::parse_num(chars.by_ref())) {
                (Ok(left), Ok(right)) => Ok(Ast::Op(op, Box::new(left), Box::new(right))),
                _ => Err(ParseAstError(0)),
            }
        } else {
            left
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ParseAstError(usize);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_num() {
        assert_eq!("5".parse(), Ok(Ast::Num(5.0)));
    }

    #[test]
    fn parsing_error() {
        assert_eq!(Ast::from_str("abc"), Err(ParseAstError(0)));
    }

    #[test]
    fn parse_addition() {
        assert_eq!(
            "4+3".parse(),
            Ok(Ast::Op(
                '+',
                Box::new(Ast::Num(4.0)),
                Box::new(Ast::Num(3.0))
            ))
        )
    }

    #[test]
    fn parse_subtraction() {
        assert_eq!(
            "5-3".parse(),
            Ok(Ast::Op(
                '-',
                Box::new(Ast::Num(5.0)),
                Box::new(Ast::Num(3.0))
            ))
        )
    }
}
