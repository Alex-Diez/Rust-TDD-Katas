pub use tdd_kata::lcd_kata::day_6::{Display, VecDigit};
pub use tdd_kata::lcd_kata::day_6::Digit::{One, Two, Three, Four, Five, Six, Seven, Eight, Nine, Zero};

pub use expectest::prelude::{be_ok, be_err};

describe! lcd_tests {

    before_each {
        let mut display = Display::new();
    }

    it "should output nothing with empty input" {
        display.input("");

        expect!(display.output()).to(be_ok().value(VecDigit::new(vec![])));
    }

    it "should output one" {
        display.input("1");

        expect!(display.output()).to(be_ok().value(VecDigit::new(vec![One])));
    }

    it "should output all digits" {
        display.input("1234567890");

        expect!(display.output()).to(be_ok().value(VecDigit::new(vec![One, Two, Three, Four, Five, Six, Seven, Eight, Nine, Zero])));
    }

    it "should show error" {
        display.input("abc");

        expect!(display.output()).to(be_err());
    }
}
