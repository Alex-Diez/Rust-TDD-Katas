pub use tdd_kata::lcd_kata::day_8::{Display, VecDigit};
pub use tdd_kata::lcd_kata::day_8::Digit::{One, Two, Three, Four, Five, Six, Seven, Eight, Nine, Zero};

pub use expectest::prelude::{be_err, be_ok};

describe! lcd_tests {

    before_each {
        let mut display = Display::new();
    }

    it "should show nothing" {
        expect!(display.output()).to(be_ok().value(VecDigit::empty()));
    }

    it "should show nothing with empty input" {
        display.input("");

        expect!(display.output()).to(be_ok().value(VecDigit::empty()));
    }

    it "should show one" {
        display.input("1");

        expect!(display.output()).to(be_ok().value(VecDigit::new(vec![One])));
    }

    it "should show all digits" {
        display.input("1234567890");

        expect!(display.output()).to(be_ok().value(VecDigit::new(vec![One, Two, Three, Four, Five, Six, Seven, Eight, Nine, Zero])));
    }

    it "should show error when non digit on input" {
        display.input("abc");

        expect!(display.output()).to(be_err());
    }
}
