extern crate calc;

use calc::evaluate;

#[test]
fn test_eval_simple_num() {
    assert_eq!(evaluate("1"), Ok(1.0));
}

#[test]
fn test_eval_three_digit_num() {
    assert_eq!(evaluate("354"), Ok(354.0));
}

#[test]
fn test_eval_real_num() {
    assert_eq!(evaluate("1523.35"), Ok(1523.35));
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
fn test_eval_few_operation() {
    assert_eq!(evaluate("10-2+3-5"), Ok(6.0));
}

#[test]
fn test_eval_mul() {
    assert_eq!(evaluate("10×3"), Ok(30.0));
}

#[test]
fn test_eval_div() {
    assert_eq!(evaluate("10÷4"), Ok(2.5));
}

#[test]
fn test_eval_operation_with_diff_priority() {
    assert_eq!(evaluate("10+2×3-50÷2"), Ok(-9.0));
}

#[test]
fn test_eval_with_parantheses() {
    assert_eq!(evaluate("10+(2+3×2)-20"), Ok(-2.0));
}

#[test]
fn test_eval_with_two_level_of_parantheses() {
    assert_eq!(evaluate("20-(2+(3-1×5)+3)+5"), Ok(22.0));
}
