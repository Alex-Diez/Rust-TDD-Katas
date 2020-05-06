use std::str::{FromStr, Chars};
use std::iter::Peekable;

#[derive(Debug, PartialEq)]
pub enum Ast {
    Number(f64),
    Op(char, Box<Ast>, Box<Ast>)
}

impl Ast {
    fn parse_num(chars: &mut Peekable<Chars>) -> Result<Ast, ParseAstError> {
        let mut num = String::new();
        let mut count = 0;
        while let Some(&char) = chars.peek() {
            count += 1;
            if vec!['+', '*', '/'].contains(&char) {
                break;
            }
            if count > 1 && char == '-' {
                break;
            }
            num.push(char);
            chars.next();
        }
        f64::from_str(num.as_str())
            .map(|num| Ast::Number(num))
            .map_err(|_| ParseAstError)
    }
}

impl FromStr for Ast {
    type Err = ParseAstError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars().peekable();
        let left = Ast::parse_num(chars.by_ref());
        let op = chars.next();
        let right = Ast::parse_num(chars.by_ref());
        if let Ok(num) = right {
            Ok(Ast::Op(op.unwrap(), Box::new(left.unwrap()), Box::new(num)))
        } else {
            left
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct ParseAstError;

#[cfg(test)]
mod tests {
    use super::*;

    fn num(n: f64) -> Ast {
        Ast::Number(n)
    }

    #[test]
    fn number() {
        assert_eq!(Ast::from_str("1"), Ok(num(1.0)))
    }

    #[test]
    fn float_number() {
        assert_eq!(Ast::from_str("-1.5"), Ok(num(-1.5)))
    }

    #[test]
    fn addition() {
        assert_eq!(
            Ast::from_str("2+3"),
            Ok(Ast::Op('+', Box::new(num(2.0)), Box::new(num(3.0))))
        )
    }

    #[test]
    fn subtraction() {
        assert_eq!(
            Ast::from_str("4-2"),
            Ok(Ast::Op('-', Box::new(num(4.0)), Box::new(num(2.0))))
        )
    }

    #[test]
    fn multiplication() {
        assert_eq!(
            Ast::from_str("5*2"),
            Ok(Ast::Op('*', Box::new(num(5.0)), Box::new(num(2.0))))
        )
    }

    #[test]
    fn division() {
        assert_eq!(
            Ast::from_str("10/3"),
            Ok(Ast::Op('/', Box::new(num(10.0)), Box::new(num(3.0))))
        )
    }
}
