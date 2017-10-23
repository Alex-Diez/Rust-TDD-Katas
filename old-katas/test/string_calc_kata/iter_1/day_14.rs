use tdd_kata::string_calc_kata::iter_1::day_14::evaluate;

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_eval_simple_num() {
        assert_eq!(evaluate("1"), Ok(1.0));
    }

    #[test]
    fn test_eval_three_digit_num() {
        assert_eq!(evaluate("256"), Ok(256.0));
    }

    #[test]
    fn test_eval_real_num() {
        assert_eq!(evaluate("125.256"), Ok(125.256));
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
    fn test_eval_few_operations() {
        assert_eq!(evaluate("2+3-1+4"), Ok(8.0));
    }

    #[test]
    fn test_eval_mul() {
        assert_eq!(evaluate("2×5"), Ok(10.0));
    }

    #[test]
    fn test_eval_div() {
        assert_eq!(evaluate("10÷2"), Ok(5.0));
    }

    #[test]
    fn test_eval_operations_with_diff_priority() {
        assert_eq!(evaluate("20+2×5-100÷4"), Ok(5.0));
    }

    #[test]
    fn test_eval_operations_with_parentheses() {
        assert_eq!(evaluate("2+(2-3+5×2)-8"), Ok(3.0));
    }

    #[test]
    fn test_eval_operations_with_two_levels_of_parentheses() {
        assert_eq!(evaluate("2+(2-3+5×2)-((1+1)×4)"), Ok(3.0));
    }
}
