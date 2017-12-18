use std::borrow::Cow;
use std::iter::Peekable;
use std::marker::PhantomData;
use std::ops::{Add, Div, Mul, Sub};
use std::str::{Chars, FromStr};

pub fn calculate<'s, N>(src: Cow<'s, str>) -> Result<N, N::Err>
    where N: Add<Output=N> + Sub<Output=N> + Mul<Output=N> + Div<Output=N> + FromStr {
    let mut eval = Evaluator::new(src.chars().peekable());
    eval.parse_expression()
}

struct Evaluator<'s, N> {
    iter: Peekable<Chars<'s>>,
    phantom: PhantomData<N>,
}

impl<'s, N> Evaluator<'s, N>
    where N: Add<Output=N> + Sub<Output=N> + Mul<Output=N> + Div<Output=N> + FromStr {
    fn new(iter: Peekable<Chars<'s>>) -> Self {
        Evaluator { iter, phantom: PhantomData }
    }

    fn parse_expression(&mut self) -> Result<N, N::Err> {
        let mut ret = self.parse_term();
        while let Some(operation) = self.resolve_low_priority_operation() {
            self.skip();
            ret = ret.and_then(|ret| self.parse_term().map(|num| operation(ret, num)));
        }
        ret
    }

    fn resolve_low_priority_operation(&mut self) -> Option<fn(N, N) -> N> {
        match self.take_ahead() {
            Some('+') => Some(add),
            Some('-') => Some(sub),
            _ => None
        }
    }

    fn parse_term(&mut self) -> Result<N, N::Err> {
        let mut ret = self.parse_num();
        while let Some(operation) = self.resolve_high_priority_operation() {
            self.skip();
            ret = ret.and_then(|ret| self.parse_num().map(|num| operation(ret, num)));
        }
        ret
    }

    fn resolve_high_priority_operation(&mut self) -> Option<fn(N, N) -> N> {
        match self.take_ahead() {
            Some('×') => Some(mul),
            Some('÷') => Some(div),
            _ => None
        }
    }

    fn parse_num(&mut self) -> Result<N, N::Err> {
        let mut num = String::new();
        loop {
            match self.iter.peek().cloned() {
                Some('+') | Some('×') | Some('÷') | Some(')') | None => break,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluate_negative_number() {
        assert_eq!(calculate(Cow::Borrowed("-8")), Ok(-8.0));
    }

    #[test]
    fn evaluate_addition() {
        assert_eq!(calculate(Cow::Borrowed("3+5")), Ok(8.0));
    }

    #[test]
    fn evaluate_subtraction() {
        assert_eq!(calculate(Cow::Borrowed("3-4")), Ok(-1.0));
    }

    #[test]
    fn evaluate_multiplication() {
        assert_eq!(calculate(Cow::Borrowed("4×5")), Ok(20.0));
    }

    #[test]
    fn evaluate_division() {
        assert_eq!(calculate(Cow::Borrowed("63÷7")), Ok(9.0));
    }

    #[test]
    fn evaluate_many_operations() {
        assert_eq!(calculate(Cow::Borrowed("3+2×4-81÷3")), Ok(-16.0));
    }

    #[test]
    fn evaluate_operations_with_parenthesis() {
        assert_eq!(calculate(Cow::Borrowed("((3+2×4)-81)÷7")), Ok(-10.0));
    }
}
