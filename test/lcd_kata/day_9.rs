pub use tdd_kata::lcd_kata::day_9::{Display, VecDigit};
pub use tdd_kata::lcd_kata::day_9::Digit::{One, Two, Three, Four, Five, Six, Seven, Eight, Nine, Zero};

pub use expectest::prelude::{be_err, be_ok};

describe! lcd_tests {

    before_each {
        let mut display = Display::new();
    }

    it "should show nothing without input" {
        expect!(display.output()).to(be_ok().value(VecDigit(vec![])));
    }

    it "should show nothing with empty input" {
        display.input("");

        expect!(display.output()).to(be_ok().value(VecDigit(vec![])));
    }

    it "should show one" {
        display.input("1");

        expect!(display.output()).to(be_ok().value(VecDigit(vec![One])));
    }

    it "should show all digits" {
        display.input("1234567890");

        expect!(display.output()).to(be_ok().value(VecDigit(vec![One, Two, Three, Four, Five, Six, Seven, Eight, Nine, Zero])));
    }

    it "should show error when input is not a number" {
        display.input("abc");

        expect!(display.output()).to(be_err());
    }
}
