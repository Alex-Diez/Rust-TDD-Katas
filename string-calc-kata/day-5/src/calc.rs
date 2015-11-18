use std::iter::Peekable;
use std::str::Chars;

pub fn evaluate(line: &str) -> f32 {
    evaluate_iter(&mut line.chars().peekable())
}

fn evaluate_iter(iter: &mut Peekable<Chars>) -> f32 {
    let mut accumulator = parse_arg(&mut iter.by_ref());
    while iter.peek().is_some() {
        let sign = iter.next();
        let next_arg = parse_arg(&mut iter.by_ref());
        match sign {
            Some('+') => accumulator += evaluate_iter(&mut iter.by_ref().take_while(|c| *c == '+' || *c == '-')),
            Some('-') => accumulator -= next_arg,
            Some('×') => accumulator *= next_arg,
            Some('÷') => accumulator /= next_arg,
            Some(_) | None => {},
        }
    }
    accumulator
}

fn parse_arg(iter: &mut Peekable<Chars>) -> f32 {
    let mut has_point = false;
    let mut accumulator = 0.0;
    let mut exponent = 0.1;
    while iter.peek().is_some() && (iter.peek().unwrap().is_digit(10) || *iter.peek().unwrap() == '.') {
        let symbol = iter.next();
        match symbol {
            Some('.') => {
                has_point = true;
                continue
            },
            Some(d @ '0'...'9') => {
                let v = d.to_digit(10).unwrap() as f32;
                if !has_point {
                    accumulator = accumulator*10.0 + v;
                }
                else {
                    accumulator = accumulator + v*exponent;
                    exponent *= 0.1;
                }
            },
            Some(_) | None => {},
        }
    }
    accumulator
}
