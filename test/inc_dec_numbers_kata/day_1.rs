pub use tdd_kata::inc_dec_numbers_kata::day_1::Checker;
pub use tdd_kata::inc_dec_numbers_kata::day_1::NumberType::{Neither, Inc, Dec};

pub use expectest::prelude::be_equal_to;

describe! inc_dec_tests {

    before_each {
        let checker = Checker::new();
    }

    it "should be neither increasing nor decrising number" {
        expect!(checker.check(0)).to(be_equal_to(Neither));
    }

    ignore "should be increasing number if it has 1 digit" {
        expect!(checker.check(7)).to(be_equal_to(Inc));
    }

    it "should be decreasing number if it has bigger digit before" {
        expect!(checker.check(61)).to(be_equal_to(Dec));
    }

    it "should be neither increasing nor decrising if it has local max inside" {
        expect!(checker.check(12321)).to(be_equal_to(Neither));
    }
}
