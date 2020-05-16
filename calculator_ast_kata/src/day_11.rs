use std::iter::{Enumerate, Peekable};
use std::str::{Chars, FromStr};

type Cursor<'c> = Peekable<Enumerate<Chars<'c>>>;

#[derive(Debug, PartialEq)]
pub enum Ast {
    Num(f64),
    Op(char, Box<Ast>, Box<Ast>),
}

impl Ast {
    fn index(cursor: &mut Cursor) -> usize {
        match cursor.peek() {
            Some(&(index, _)) => index,
            None => 0,
        }
    }

    fn parse_num(cursor: &mut Cursor) -> Result<Self, ParseAstError> {
        let mut num = String::new();
        let index = Self::index(cursor.by_ref());
        while let Some(&(index, char)) = cursor.peek() {
            match char {
                '+' | '*' | '/' => break,
                '-' if !num.is_empty() => break,
                '0'..='9' | '-' | '.' => {
                    cursor.next();
                    num.push(char);
                }
                _ => return Err(ParseAstError(index)),
            }
        }
        num.parse().map(Ast::Num).map_err(|_| ParseAstError(index))
    }

    fn parse_high_priority_op(cursor: &mut Cursor) -> Option<char> {
        match cursor.peek() {
            Some(&(_, '*')) | Some(&(_, '/')) => cursor.next().map(|(_, op)| op),
            _ => None,
        }
    }

    fn parse_term(cursor: &mut Cursor) -> Result<Self, ParseAstError> {
        let mut root = Self::parse_num(cursor.by_ref());
        while let Some(op) = Self::parse_high_priority_op(cursor.by_ref()) {
            root = match (root, Self::parse_num(cursor.by_ref())) {
                (Ok(left), Ok(right)) => Ok(Ast::Op(op, Box::new(left), Box::new(right))),
                (_, Err(right)) => Err(right),
                (Err(left), _) => Err(left),
            }
        }
        root
    }

    fn parse_low_priority_op(cursor: &mut Cursor) -> Option<char> {
        match cursor.peek() {
            Some(&(_, '+')) | Some(&(_, '-')) => cursor.next().map(|(_, op)| op),
            _ => None,
        }
    }

    fn parse_expression(cursor: &mut Cursor) -> Result<Self, ParseAstError> {
        let mut root = Self::parse_term(cursor.by_ref());
        while let Some(op) = Self::parse_low_priority_op(cursor.by_ref()) {
            root = match (root, Self::parse_term(cursor.by_ref())) {
                (Ok(left), Ok(right)) => Ok(Ast::Op(op, Box::new(left), Box::new(right))),
                (_, Err(right)) => Err(right),
                (Err(left), _) => Err(left),
            }
        }
        root
    }
}

impl FromStr for Ast {
    type Err = ParseAstError;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        let mut chars = source.chars().enumerate().peekable();
        Self::parse_expression(chars.by_ref())
    }
}

#[derive(Debug, PartialEq)]
pub struct ParseAstError(usize);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error() {
        assert_eq!(Ast::from_str("abc"), Err(ParseAstError(0)))
    }

    #[test]
    fn number() {
        assert_eq!("5".parse(), Ok(Ast::Num(5.0)))
    }

    #[test]
    fn float_number() {
        assert_eq!("4.23".parse(), Ok(Ast::Num(4.23)))
    }

    #[test]
    fn negative_number() {
        assert_eq!("-5".parse(), Ok(Ast::Num(-5.0)))
    }

    #[test]
    fn addition() {
        assert_eq!(
            "5+3".parse(),
            Ok(Ast::Op(
                '+',
                Box::new(Ast::Num(5.0)),
                Box::new(Ast::Num(3.0))
            ))
        )
    }

    #[test]
    fn left_hand_side_error() {
        assert_eq!(Ast::from_str("abc+2"), Err(ParseAstError(0)))
    }

    #[test]
    fn right_hand_side_error() {
        assert_eq!(Ast::from_str("4+abc"), Err(ParseAstError(2)))
    }

    #[test]
    fn subtraction() {
        assert_eq!(
            "6-4".parse(),
            Ok(Ast::Op(
                '-',
                Box::new(Ast::Num(6.0)),
                Box::new(Ast::Num(4.0))
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
            "9/3".parse(),
            Ok(Ast::Op(
                '/',
                Box::new(Ast::Num(9.0)),
                Box::new(Ast::Num(3.0))
            ))
        )
    }

    #[test]
    fn many_operations() {
        assert_eq!(
            "5+3*5-4/2".parse(),
            Ok(Ast::Op(
                '-',
                Box::new(Ast::Op(
                    '+',
                    Box::new(Ast::Num(5.0)),
                    Box::new(Ast::Op(
                        '*',
                        Box::new(Ast::Num(3.0)),
                        Box::new(Ast::Num(5.0))
                    ))
                )),
                Box::new(Ast::Op(
                    '/',
                    Box::new(Ast::Num(4.0)),
                    Box::new(Ast::Num(2.0))
                ))
            ))
        )
    }

    #[test]
    fn unknown_operator() {
        assert_eq!(Ast::from_str("5&4"), Err(ParseAstError(1)))
    }
}
