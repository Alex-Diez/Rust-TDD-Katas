use tdd_kata::string_calc_kata::day_5::evaluate;

#[test]
fn test_evaluate_simple_number() {
    assert_eq!(evaluate("1"), 1.0);
}

#[test]
fn test_evaluate_big_number() {
    assert_eq!(evaluate("100"), 100.0);
}

#[test]
fn test_evaluate_real_number() {
    assert_eq!(evaluate("1.09"), 1.09)
}

#[test]
fn test_evaluate_add() {
    assert_eq!(evaluate("1.09+1.01"), 2.1);
}

#[test]
fn test_evaluate_sub() {
    assert_eq!(evaluate("2-1"), 1.0);
}

#[test]
fn test_evaluate_mul() {
    assert_eq!(evaluate("2×2"), 4.0);
}

#[test]
fn test_evaluate_div() {
    assert_eq!(evaluate("22÷2"), 11.0);
}

#[test]
fn test_two_adds() {
    assert_eq!(evaluate("2+3+6"), 11.0);
}

#[test]
fn test_two_subs() {
    assert_eq!(evaluate("6-4-1"), 1.0);
}

#[test]
fn test_operation_with_different_priority() {
    assert_eq!(evaluate("2+3×2"), 8.0);
}
