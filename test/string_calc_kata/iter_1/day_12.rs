use tdd_kata::string_calc_kata::iter_1::day_12::evaluate;

#[test]
fn test_eval_simle_num() {
    assert_eq!(evaluate("1"), Ok(1.0));
}

#[test]
fn test_eval_three_digit_num() {
    assert_eq!(evaluate("123"), Ok(123.0));
}

#[test]
fn test_eval_real_num() {
    assert_eq!(evaluate("12.256"), Ok(12.256));
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
fn test_eval_few_operation() {
    assert_eq!(evaluate("2+3-1"), Ok(4.0));
}

#[test]
fn test_eval_mul() {
    assert_eq!(evaluate("2×3"), Ok(6.0));
}

#[test]
fn test_eval_div() {
    assert_eq!(evaluate("10÷2"), Ok(5.0));
}

#[test]
fn test_eval_operation_with_diffrenet_priorities() {
    assert_eq!(evaluate("1+2×5-6÷2-4"), Ok(4.0));
}

#[test]
fn test_eval_with_parentheses() {
    assert_eq!(evaluate("1+(5-3)×2-(6+1)"), Ok(-2.0));
}

#[test]
fn test_eval_with_two_level_of_parentheses() {
    assert_eq!(evaluate("1+(5-(2+1)+4)×3-(4+2)"), Ok(13.0));
}
