use tdd_kata::string_calc_kata::iter_1::day_6::evaluate;

#[test]
fn test_eval_simple_num() {
    assert_eq!(evaluate("1"), 1.0);
}

#[test]
fn test_eval_three_digit_num() {
    assert_eq!(evaluate("100"), 100.0);
}

#[test]
fn test_eval_real_num() {
    assert_eq!(evaluate("1.01"), 1.01)
}

#[test]
fn test_eval_add() {
    assert_eq!(evaluate("1+1"), 2.0);
}

#[test]
fn test_eval_sub() {
    assert_eq!(evaluate("2-1"), 1.0);
}

#[test]
fn test_eval_mul() {
    assert_eq!(evaluate("2×3"), 6.0);
}

#[test]
fn test_eval_div() {
    assert_eq!(evaluate("6÷2"), 3.0);
}

#[test]
fn test_eval_two_operation() {
    assert_eq!(evaluate("3+2-1"), 4.0);
}

#[test]
fn test_eval_operation_with_diff_priority() {
    assert_eq!(evaluate("2+2×2-2+6÷3"), 6.0);
}
