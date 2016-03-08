use std::iter::Peekable;
use std::str::Chars;

pub fn evaluate(line: String) -> f32 {
    let mut iter = line.chars().peekable();
    let mut accumulator = parse_arg(&mut iter);
    while iter.peek().is_some() {
        let sign = iter.next();
        match sign {
            Some('+') => accumulator += evaluate(iter.by_ref().collect()),
            Some('-') => accumulator -= evaluate(iter.by_ref().collect()),
            Some('×') => accumulator *= parse_arg(&mut iter),
            Some('÷') => accumulator /= parse_arg(&mut iter),
            Some(_) => accumulator += 0.0,
            None => accumulator += 0.0,
        }
    }
    accumulator
}

fn parse_arg(iter: &mut Peekable<Chars>) -> f32 {
    let mut accumulator = 0.0;
    let mut exponent = 0.1;
    let mut has_point = false;
    while iter.peek().is_some() && is_number_symbol(iter.peek().unwrap()) {
        let c = iter.next().unwrap();
        if c == '.' {
            has_point = true;
            continue;
        }
        let value = c.to_digit(10).unwrap() as f32;
        if has_point {
            accumulator = accumulator + value * exponent;
            exponent = exponent * 0.1;
        }
        else {
            accumulator = accumulator*10.0 + value;
        }
    }
    accumulator
}

fn is_number_symbol(c: &char) -> bool {
    c.is_digit(10) || *c == '.'
}
