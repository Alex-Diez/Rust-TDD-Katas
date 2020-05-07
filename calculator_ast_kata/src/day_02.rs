use std::str::{FromStr, Chars};
use std::iter::Peekable;

#[derive(Debug, PartialEq)]
enum Ast {
    Num(f64),
    Op(char, Box<Ast>, Box<Ast>)
}

impl Ast {
    fn parse_num(chars: &mut Peekable<Chars>) -> Result<Self, ParseAstError> {
        let mut num = String::new();
        while let Some(char) = chars.peek() {
            match char {
                &'+' | &'*' | &'/' => break,
                &'-' if !num.is_empty() => break,
                &char => {
                    chars.next();
                    num.push(char);
                }
            }
        }
        f64::from_str(num.as_str())
            .map(Ast::Num)
            .map_err(|_| ParseAstError)
    }

    fn parse_high_op(chars: &mut Peekable<Chars>) -> Option<char> {
        match chars.peek() {
            Some(&'*') | Some(&'/') => chars.next(),
            _ => None
        }
    }
    
    fn parse_term(chars: &mut Peekable<Chars>) -> Result<Ast, ParseAstError> {
        let mut root = Ast::parse_num(chars.by_ref());
        while let Some(op) = Ast::parse_high_op(chars.by_ref()) {
            root = Ok(
                Ast::Op(
                    op,
                    Box::new(root.unwrap()),
                    Box::new(Ast::parse_num(chars.by_ref()).unwrap())
                )
            )
        }
        root
    }

    fn parse_low_op(chars: &mut Peekable<Chars>) -> Option<char> {
        match chars.peek() {
            Some(&'+') | Some(&'-') => chars.next(),
            _ => None
        }
    }

    fn parse_expresion(chars: &mut Peekable<Chars>) -> Result<Ast, ParseAstError> {
        let mut root = Ast::parse_term(chars.by_ref());
        while let Some(op) = Ast::parse_low_op(chars.by_ref()) {
            root = Ok(
                Ast::Op(
                    op,
                    Box::new(root.unwrap()),
                    Box::new(Ast::parse_term(chars.by_ref()).unwrap())
                )
            )
        }
        root
    }
}

impl FromStr for Ast {
    type Err = ParseAstError;

    fn from_str(src: &str) -> Result<Self, Self::Err> {
        let mut chars = src.chars().peekable();
        Ast::parse_expresion(chars.by_ref())
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
        assert_eq!(Ast::from_str("1"), Ok(Ast::Num(1.0)));
    }

    #[test]
    fn negative_numbers() {
        assert_eq!(Ast::from_str("-4"), Ok(Ast::Num(-4.0)));
    }

    #[test]
    fn addition() {
        assert_eq!(
            Ast::from_str("2+3"),
            Ok(Ast::Op('+', Box::new(Ast::Num(2.0)), Box::new(Ast::Num(3.0))))
        )
    }

    #[test]
    fn subtraction() {
        assert_eq!(
            Ast::from_str("4-3"),
            Ok(Ast::Op('-', Box::new(Ast::Num(4.0)), Box::new(Ast::Num(3.0))))
        )
    }

    #[test]
    fn multiplication() {
        assert_eq!(
            Ast::from_str("5*3"),
            Ok(Ast::Op('*', Box::new(Ast::Num(5.0)), Box::new(Ast::Num(3.0))))
        )
    }

    #[test]
    fn division() {
        assert_eq!(
            Ast::from_str("10/3"),
            Ok(Ast::Op('/', Box::new(Ast::Num(10.0)), Box::new(Ast::Num(3.0))))
        )
    }

    #[test]
    fn many_operations() {
        assert_eq!(
            Ast::from_str("4+5*3-6/2"),
            Ok(
                Ast::Op(
                    '-',
                    Box::new(Ast::Op(
                        '+',
                        Box::new(Ast::Num(4.0)),
                        Box::new(Ast::Op(
                            '*',
                            Box::new(Ast::Num(5.0)),
                            Box::new(Ast::Num(3.0))
                        )))),
                    Box::new(Ast::Op(
                        '/',
                        Box::new(Ast::Num(6.0)),
                        Box::new(Ast::Num(2.0))
                    ))
                )
            )
        )
    }
}
