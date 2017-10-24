use tdd_kata::string_calc_kata::iter_1::day_4::evaluate;

#[test]
fn test_evaluate_one() {
    assert_eq!(evaluate("1".to_string()), 1.0);
}

#[test]
fn test_evaluate_one_hundred() {
    assert_eq!(evaluate("100".to_string()), 100.0);
}

#[test]
fn test_evaluate_add() {
    assert_eq!(evaluate("5+4".to_string()), 9.0);
}

#[test]
fn test_evaluate_sub() {
    assert_eq!(evaluate("5-4".to_string()), 1.0);
}

#[test]
fn test_evaluate_mul() {
    assert_eq!(evaluate("5×4".to_string()), 20.0);
}

#[test]
fn test_evaluate_div() {
    assert_eq!(evaluate("20÷5".to_string()), 4.0);
}

#[test]
fn test_evaluate_float_number() {
    assert_eq!(evaluate("100.254".to_string()), 100.254)
}

#[test]
fn test_evaluate_two_add() {
    assert_eq!(evaluate("10+11+5".to_string()), 26.0);
}

#[test]
#[ignore]
fn test_evaluate_two_sub() {
    assert_eq!(evaluate("20-11-5".to_string()), 4.0);
}

#[test]
fn test_evaluate_operation_with_differenet_priority() {
    assert_eq!(evaluate("2+2×2".to_string()), 6.0);
}
