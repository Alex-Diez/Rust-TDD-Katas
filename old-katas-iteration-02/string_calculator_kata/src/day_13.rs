use std::borrow::Cow;
use std::iter::Peekable;
use std::str::{Chars, FromStr};
use std::ops::{Add, Sub, Mul, Div};

pub fn calculate<'s, N>(src: Cow<'s, str>) -> Result<N, N::Err>
    where N: Add<Output=N> + Sub<Output=N> + Mul<Output=N> + Div<Output=N> + FromStr {
    Evaluator::new(src.chars().peekable()).parse_expression()
}

fn add<N: Add<Output=N>>(acc: N, num: N) -> N {
    acc + num
}

fn sub<N: Sub<Output=N>>(acc: N, num: N) -> N {
    acc - num
}

fn mul<N: Mul<Output=N>>(acc: N, num: N) -> N {
    acc * num
}

fn div<N: Div<Output=N>>(acc: N, num: N) -> N {
    acc / num
}

use std::marker::PhantomData;

struct Evaluator<'e, N> {
    iter: Peekable<Chars<'e>>,
    phantom: PhantomData<N>,
}

impl<'e, N> Evaluator<'e, N>
    where N: Add<Output=N> + Sub<Output=N> + Mul<Output=N> + Div<Output=N> + FromStr {
    fn new(iter: Peekable<Chars<'e>>) -> Self {
        Evaluator { iter, phantom: PhantomData }
    }

    fn parse_expression(&mut self) -> Result<N, N::Err> {
        let mut ret = self.parse_term();
        while let Some(operation) = self.resolve_low_order_operation() {
            ret = ret.and_then(|ret| self.parse_term().map(|num| operation(ret, num)))
        }
        ret
    }

    fn resolve_low_order_operation(&mut self) -> Option<fn(N, N) -> N> {
        match self.take_ahead() {
            Some('+') => {
                self.skip();
                Some(add)
            }
            Some('-') => {
                self.skip();
                Some(sub)
            }
            _ => None
        }
    }

    fn parse_term(&mut self) -> Result<N, N::Err> {
        let mut ret = self.parse_num();
        while let Some(operation) = self.resolve_high_order_operation() {
            ret = ret.and_then(|ret| self.parse_num().map(|num| operation(ret, num)))
        }
        ret
    }

    fn resolve_high_order_operation(&mut self) -> Option<fn(N, N) -> N> {
        match self.take_ahead() {
            Some('×') => {
                self.skip();
                Some(mul)
            }
            Some('÷') => {
                self.skip();
                Some(div)
            }
            _ => None
        }
    }

    fn parse_num(&mut self) -> Result<N, N::Err> {
        let mut num = String::new();
        loop {
            match self.iter.peek().cloned() {
                Some('+') | Some('×') | Some(')') | Some('÷') | None => break,
                Some('-') if !num.is_empty() => break,
                Some('(') => {
                    self.skip();
                    let ret = self.parse_expression();
                    self.skip();
                    return ret;
                }
                Some(d) => num.push(d)
            }
            self.iter.next();
        }
        num.parse()
    }

    fn take_ahead(&mut self) -> Option<char> {
        self.iter.peek().cloned()
    }

    fn skip(&mut self) {
        self.iter.next();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluate_negative_number() {
        assert_eq!(calculate(Cow::Borrowed("-41")), Ok(-41.0));
    }

    #[test]
    fn evaluate_addition() {
        assert_eq!(calculate(Cow::Borrowed("13+12")), Ok(25.0));
    }

    #[test]
    fn evaluate_subtraction() {
        assert_eq!(calculate(Cow::Borrowed("21-19")), Ok(2.0));
    }

    #[test]
    fn evaluate_multiplication() {
        assert_eq!(calculate(Cow::Borrowed("3×5")), Ok(15.0));
    }

    #[test]
    fn evaluate_division() {
        assert_eq!(calculate(Cow::Borrowed("21÷3")), Ok(7.0));
    }

    #[test]
    fn evaluate_many_operation() {
        assert_eq!(calculate(Cow::Borrowed("4+2×3-16÷4")), Ok(6.0));
    }

    #[test]
    fn evaluate_operations_with_parenthesis() {
        assert_eq!(calculate(Cow::Borrowed("((4+2)×3-16)÷4")), Ok(0.5));
    }
}
