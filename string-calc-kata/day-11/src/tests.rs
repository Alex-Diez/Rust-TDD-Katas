extern crate calc;

use calc::evaluate;

#[test]
fn test_eval_simple_num() {
    assert_eq!(evaluate("1"), Ok(1.0));
}

#[test]
fn test_eval_big_num() {
    assert_eq!(evaluate("10000"), Ok(10000.0));
}

#[test]
fn test_eval_real_num() {
    assert_eq!(evaluate("10.254"), Ok(10.254));
}

#[test]
fn test_eval_add() {
    assert_eq!(evaluate("10+2"), Ok(12.0));
}

#[test]
fn test_eval_sub() {
    assert_eq!(evaluate("10-2"), Ok(8.0));
}

#[test]
fn test_eval_mul() {
    assert_eq!(evaluate("10×2"), Ok(20.0));
}

#[test]
fn test_eval_div() {
    assert_eq!(evaluate("10÷2"), Ok(5.0));
}

#[test]
fn test_eval_few_op() {
    assert_eq!(evaluate("10+2-1"), Ok(11.0));
}

#[test]
fn test_eval_op_with_diff_priority() {
    assert_eq!(evaluate("10+6×3-5-25÷5"), Ok(18.0));
}

#[test]
fn test_eval_with_parentheses() {
    assert_eq!(evaluate("2+(5-3)×3+5-1"), Ok(12.0));
}

#[test]
fn test_eval_with_two_levels_parentheses() {
    assert_eq!(evaluate("2+(5-(3-1))×3+5-1"), Ok(15.0));
}
