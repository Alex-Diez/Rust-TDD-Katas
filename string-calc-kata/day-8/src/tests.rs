extern crate calc;

use calc::evaluate;

#[test]
fn test_eval_simple_num() {
    assert_eq!(evaluate("1"), Ok(1.0));
}

#[test]
fn test_eval_three_digit_num() {
    assert_eq!(evaluate("100"), Ok(100.0));
}

#[test]
fn test_eval_real_num() {
    assert_eq!(evaluate("10.25"), Ok(10.25));
}

#[test]
fn test_eval_add() {
    assert_eq!(evaluate("1+2"), Ok(3.0));
}

#[test]
fn test_eval_sub() {
    assert_eq!(evaluate("2-1"), Ok(1.0));
}

#[test]
fn test_eval_mul() {
    assert_eq!(evaluate("2×3"), Ok(6.0));
}

#[test]
fn test_eval_div() {
    assert_eq!(evaluate("6÷2"), Ok(3.0));
}

#[test]
fn test_eval_fiew_operation() {
    assert_eq!(evaluate("6+8-10"), Ok(4.0));
}

#[test]
fn test_eval_operation_with_diff_priority() {
    assert_eq!(evaluate("6+8×3-10÷2"), Ok(25.0))
}
