pub use tdd_kata::lcd_kata::day_7::{Display, VecDigit};
pub use tdd_kata::lcd_kata::day_7::Digit::{One, Two, Three, Four, Five, Six, Seven, Eight, Nine, Zero};

pub use expectest::prelude::{be_err, be_ok};

describe! lcd_tests {

    before_each {
        let mut display = Display::new();
    }

    it "should print nothing" {
        expect!(display.output()).to(be_ok().value(VecDigit::empty()));
    }

    it "should print nothing with empty input" {
        display.input("");

        expect!(display.output()).to(be_ok().value(VecDigit::empty()));
    }

    it "should print one" {
        display.input("1");

        expect!(display.output()).to(be_ok().value(VecDigit::new(vec![One])));
    }

    it "should print all digits" {
        display.input("1234567890");

        expect!(display.output()).to(be_ok().value(VecDigit::new(vec![One, Two, Three, Four, Five, Six, Seven, Eight, Nine, Zero])));
    }

    it "should print error" {
        display.input("abc");

        expect!(display.output()).to(be_err());
    }
}
