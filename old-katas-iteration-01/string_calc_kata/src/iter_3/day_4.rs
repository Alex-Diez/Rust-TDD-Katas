use std::borrow::Cow;
use std::str::Chars;
use std::iter::Peekable;

pub fn evaluate<'s>(src: Cow<'s, str>) -> f64 {
    let mut chars = (*src).chars().peekable();
    parse_expression(chars.by_ref())
}

fn parse_expression(chars: &mut Peekable<Chars>) -> f64 {
    let mut result = parse_term(chars.by_ref());
    loop {
        match chars.peek().cloned() {
            Some('+') => {
                chars.next();
                result += parse_term(chars.by_ref());
            },
            Some('-') => {
                chars.next();
                result -= parse_term(chars.by_ref());
            },
            _ => break
        }
    }
    result
}

fn parse_term(chars: &mut Peekable<Chars>) -> f64 {
    let mut result = parse_arg(chars.by_ref());
    loop {
        match chars.peek().cloned() {
            Some('×') => {
                chars.next();
                result *= parse_arg(chars.by_ref());
            },
            Some('÷') => {
                chars.next();
                result /= parse_arg(chars.by_ref());
            },
            _ => break
        }
    }
    result
}

fn parse_arg(chars: &mut Peekable<Chars>) -> f64 {
    match chars.peek().cloned() {
        Some('(') => {
            chars.next();
            let ret = parse_expression(chars);
            chars.next();
            ret
        },
        _ => {
            let mut arg = 0.0;

            while let Some(digit) = chars.peek().cloned().and_then(|c| c.to_digit(10)) {
                arg = 10.0 * arg + (digit as f64);
                chars.next();
            }
            arg
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::Cow;

    #[test]
    fn evaluate_num() {
        assert_eq!(evaluate(Cow::Borrowed("10")), 10.0);
    }

    #[test]
    fn evaluate_add() {
        assert_eq!(evaluate(Cow::Borrowed("1+2")), 3.0);
    }

    #[test]
    fn evaluate_sub() {
        assert_eq!(evaluate(Cow::Borrowed("4-1")), 3.0);
    }

    #[test]
    fn evaluate_mul() {
        assert_eq!(evaluate(Cow::Borrowed("5×6")), 30.0);
    }

    #[test]
    fn evaluate_div() {
        assert_eq!(evaluate(Cow::Borrowed("40÷10")), 4.0);
    }

    #[test]
    fn evaluate_multiple_operations() {
        assert_eq!(evaluate(Cow::Borrowed("4+10÷2-3×4")), -3.0);
    }

    #[test]
    fn evaluate_expression_with_parenthesis() {
        assert_eq!(evaluate(Cow::Borrowed("(4+10)÷2-3×((4+6)-2)")), -17.0);
    }
}
