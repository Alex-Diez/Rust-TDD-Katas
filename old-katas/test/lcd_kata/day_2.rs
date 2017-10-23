pub use tdd_kata::lcd_kata::day_2::Display;
pub use tdd_kata::lcd_kata::day_2::Number::*;

pub use expectest::prelude::{be_err, be_ok};

describe! lcd_tests {

    before_each {
        let mut display = Display::new();
    }

    it "should output nothing on empty input" {
        display.input("");

        expect!(display.output()).to(be_ok().value(vec![]));
    }

    it "should output one" {
        display.input("1");

        expect!(display.output()).to(be_ok().value(vec![One]));
    }

    it "should output input data" {
        display.input("1234567890");

        expect!(display.output()).to(be_ok().value(vec![One, Two, Three, Four, Five, Six, Seven, Eight, Nine, Zero]));
    }

    it "should show error on non number input" {
        display.input("abc");

        expect!(display.output()).to(be_err());
    }
}
