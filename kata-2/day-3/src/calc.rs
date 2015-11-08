use std::iter::Peekable;
use std::str::Chars;
use std::option::Option;

pub fn evaluate(line: &str) -> f32 {
    let mut peekable = line.chars().peekable();
    let mut accumulator = parse_arg(&mut peekable);
    while peekable.peek().is_some() {
        let sign = parse_sign(&mut peekable);
        accumulator = match sign {
            Some('+') => accumulator + parse_arg(&mut peekable),
            Some('-') => accumulator - parse_arg(&mut peekable),
            Some('x') => accumulator * parse_arg(&mut peekable),
            Some('÷') => accumulator / parse_arg(&mut peekable),
            Some(_) => 0.0,
            None => accumulator
        }
    }
    accumulator
}

fn parse_arg(peekable: &mut Peekable<Chars>) -> f32 {
    let mut accumulator = 0.0;
    let mut point = false;
    let mut exponent = 0.1;
    while peekable.peek().is_some() && is_number_symbol(peekable.peek().unwrap()) {
        let v = peekable.next().unwrap();
        if v == '.' {
            point = true;
            continue;
        }
        let digit = v.to_digit(10).unwrap() as f32;
        if !point {
            accumulator = accumulator*10.0 + digit;
        }
        else {
            accumulator = accumulator + digit*exponent;
            exponent = exponent * 0.1;
        }
    }
    accumulator
}

fn parse_sign(peekable: &mut Peekable<Chars>) -> Option<char> {
    peekable.next()
}

fn is_number_symbol(c: &char) -> bool {
    c.is_digit(10) || *c == '.'
}
