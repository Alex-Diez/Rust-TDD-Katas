use std::borrow::Cow;
use std::iter::Peekable;
use std::str::Chars;
use std::collections::HashMap;

pub fn evaluate<'s>(src: Cow<'s, str>) -> f64 {
    let ops = create_operations_map();
    let mut output = vec![];
    let mut operands = vec![];
    let mut chars = (*src).chars().peekable();
    output.push(parse_arg(chars.by_ref()));
    chars.next().map(|c| { operands.push(c); });
    if chars.peek().is_some() {
        output.push(parse_arg(chars.by_ref()));
    }
    let mut ret = output.pop();
    for op in operands.into_iter().rev() {
        ret = output.pop().and_then(|arg| (*ops[&op])(arg, ret));
    }
    ret.unwrap()
}

fn parse_arg(chars: &mut Peekable<Chars>) -> f64 {
    let mut arg = 0.0;
    while let Some(digit) = chars.peek().cloned().and_then(|c| c.to_digit(10)) {
        arg = arg * 10.0 + digit as f64;
        chars.next();
    }
    arg
}

fn create_operations_map() -> HashMap<char, Box<Fn(f64, Option<f64>) -> Option<f64>>> {
    let mut ops: HashMap<char, Box<Fn(f64, Option<f64>) -> Option<f64>>> = HashMap::new();
    ops.insert('+', Box::new(|arg_one, arg_two| { arg_two.map(|arg| { arg_one + arg }) }));
    ops.insert('-', Box::new(|arg_one, arg_two| { arg_two.map(|arg| { arg_one - arg }) }));
    ops.insert('×', Box::new(|arg_one, arg_two| { arg_two.map(|arg| { arg_one * arg }) }));
    ops.insert('÷', Box::new(|arg_one, arg_two| { arg_two.map(|arg| { arg_one / arg }) }));
    ops
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
        assert_eq!(evaluate(Cow::Borrowed("10+12")), 22.0);
    }

    #[test]
    fn evaluate_sub() {
        assert_eq!(evaluate(Cow::Borrowed("14-3")), 11.0);
    }

    #[test]
    fn evaluate_mul() {
        assert_eq!(evaluate(Cow::Borrowed("4×5")), 20.0);
    }

    #[test]
    fn evaluate_div() {
        assert_eq!(evaluate(Cow::Borrowed("45÷5")), 9.0);
    }
}
