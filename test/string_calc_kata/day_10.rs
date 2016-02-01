use tdd_kata::string_calc_kata::day_10::evaluate;

#[test]
fn test_eval_simple_num() {
    assert_eq!(evaluate("1"), Ok(1.0));
}

#[test]
fn test_eval_three_digit_num() {
    assert_eq!(evaluate("123"), Ok(123.0));
}

#[test]
fn test_eval_real_num() {
    assert_eq!(evaluate("12.035"), Ok(12.035));
}

#[test]
fn test_eval_add() {
    assert_eq!(evaluate("1+2"), Ok(3.0));
}

#[test]
fn test_eval_sub() {
    assert_eq!(evaluate("3-1"), Ok(2.0));
}

#[test]
fn test_eval_mul() {
    assert_eq!(evaluate("3×3"), Ok(9.0));
}

#[test]
fn test_eval_div() {
    assert_eq!(evaluate("10÷2"), Ok(5.0));
}

#[test]
fn test_eval_few_operations() {
    assert_eq!(evaluate("2+3-1+10"), Ok(14.0));
}

#[test]
fn test_eval_operations_with_diff_priority() {
    assert_eq!(evaluate("2+3×9-20÷5+1"), Ok(26.0));
}

#[test]
fn test_eval_operation_with_parentheses() {
    assert_eq!(evaluate("2+(7-4)×3-10"), Ok(1.0));
}
