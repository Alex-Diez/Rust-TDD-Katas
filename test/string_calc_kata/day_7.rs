use tdd_kata::string_calc_kata::day_7::evaluate;

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
    assert_eq!(evaluate("10.025"), Ok(10.025));
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
fn test_eval_two_operation() {
    assert_eq!(evaluate("3+2-1"), Ok(4.0));
}

#[test]
fn test_eval_operation_with_diff_priority() {
    assert_eq!(evaluate("2+2×2-2+6÷3"), Ok(6.0));
}

#[test]
fn test_eval_with_parentheses() {
    assert_eq!(evaluate("2+(3-1)×6-2"), Ok(12.0));
}
