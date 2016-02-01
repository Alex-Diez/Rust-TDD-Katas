use tdd_kata::string_calc_kata::day_9::evaluate;

#[test]
fn test_eval_simple_num() {
    assert_eq!(evaluate("1"), Ok(1.0));
}

#[test]
fn test_three_digit_num() {
    assert_eq!(evaluate("100"), Ok(100.0));
}

#[test]
fn test_eval_real_num() {
    assert_eq!(evaluate("10.25"), Ok(10.25));
}

#[test]
fn test_eval_add() {
    assert_eq!(evaluate("2+1"), Ok(3.0));
}

#[test]
fn test_eval_sub() {
    assert_eq!(evaluate("2-1"), Ok(1.0));
}

#[test]
fn test_eval_mul() {
    assert_eq!(evaluate("2×4"), Ok(8.0));
}

#[test]
fn test_eval_div() {
    assert_eq!(evaluate("6÷2"), Ok(3.0));
}

#[test]
fn test_eval_few_operation() {
    assert_eq!(evaluate("5+8-3+1"), Ok(11.0));
}

#[test]
fn test_eval_operation_with_diff_priority() {
    assert_eq!(evaluate("5+9÷3-2×2+8"), Ok(12.0));
}

#[test]
fn test_eval_operation_with_parentheses() {
    assert_eq!(evaluate("2+(7-5)×3-10"), Ok(-2.0));
}
