pub use tdd_kata::lcd_kata::day_10::{Display, VecDigit};
pub use tdd_kata::lcd_kata::day_10::Digit::{One, Two, Three, Four, Five, Six, Seven, Eight, Nine, Zero};

pub use expectest::prelude::{be_err, be_ok};

describe! lcd_tests {

    before_each {
        let mut displaly = Display::new();
    }

    it "should show nothing without any input" {
        expect!(displaly.output()).to(be_ok().value(VecDigit(vec![])));
    }

    it "should show nothing with empty input" {
        displaly.input("");

        expect!(displaly.output()).to(be_ok().value(VecDigit(vec![])));
    }

    it "should show one" {
        displaly.input("1");

        expect!(displaly.output()).to(be_ok().value(VecDigit(vec![One])));
    }

    it "should show all digits" {
        displaly.input("1234567890");

        expect!(displaly.output()).to(be_ok().value(VecDigit(vec![One, Two, Three, Four, Five, Six, Seven, Eight, Nine, Zero])));
    }

    it "should show error when input is not digits' stream" {
        displaly.input("abc");

        expect!(displaly.output()).to(be_err());
    }
}
