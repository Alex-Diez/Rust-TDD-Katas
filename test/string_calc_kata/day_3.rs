use tdd_kata::string_calc_kata::day_3::evaluate;

#[test]
fn test_evaluate_simple_number() {
    assert_eq!(evaluate("1"), 1.0);
}

#[test]
fn test_evaluate_long_number() {
    assert_eq!(evaluate("100"), 100.0);
}

#[test]
fn test_evaluate_one_plus_two() {
    assert_eq!(evaluate("1+2"), 3.0);
}

#[test]
fn test_evaluate_two_minus_one() {
    assert_eq!(evaluate("2-1"), 1.0);
}

#[test]
fn test_evaluate_multiplication() {
    assert_eq!(evaluate("2x4"), 8.0);
}

#[test]
fn test_evalaute_division() {
    assert_eq!(evaluate("4÷2"), 2.0);
}

#[test]
fn test_evaluate_real_number() {
    assert_eq!(evaluate("1.012"), 1.012);
}

#[test]
fn test_evaluate_three_plus_four_plus_two() {
    assert_eq!(evaluate("3+4+2"), 9.0);
}

#[test]
#[ignore]
fn test_evaluate_operation_with_diff_priority() {
    assert_eq!(evaluate("3+4x2"), 11.0);
}
