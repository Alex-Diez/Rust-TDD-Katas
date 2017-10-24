use std::str::Chars;

pub struct Calculator<'a> {
    line: &'a str,
    operands: Vec<char>,
    sign: Option<char>
}

impl<'a> Calculator<'a> {
    pub fn new(line: &'a str) -> Calculator {
        Calculator {
            line: line,
            operands: vec!['+', '-'],
            sign: None
        }
    }

    pub fn evaluate(&mut self) -> u32 {
        let mut chars = self.line.chars();
        let first_arg = self.parse_arg(chars.by_ref());
        let sign = self.sign;
        let second_arg = self.parse_arg(chars.by_ref());
        match sign {
            Some('+') => first_arg + second_arg,
            Some('-') => first_arg - second_arg,
            _ => first_arg,
        }
    }

    fn parse_arg(&mut self, chars: &mut Chars) -> u32 {
        chars.by_ref()
            .take_while(|c| if !self.operands.contains(c) { true } else { self.sign = Some(*c); false } )
            .map(|c| c.to_digit(10).unwrap())
            .fold(0, |accumulator, i| accumulator * 10 + i)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_number() {
        let mut calc1 = Calculator::new("1");
        let mut calc2 = Calculator::new("2");

        assert_eq!(calc1.evaluate(), 1);
        assert_eq!(calc2.evaluate(), 2);
    }

    #[test]
    fn test_evaluate_one_plus_two() {
        let mut calc = Calculator::new("1+2");

        assert_eq!(calc.evaluate(), 3);
    }

    #[test]
    fn test_evaluate_two_minus_one() {
        let mut calc = Calculator::new("2-1");

        assert_eq!(calc.evaluate(), 1);
    }
}
