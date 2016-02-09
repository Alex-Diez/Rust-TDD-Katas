pub use tdd_kata::lcd_kata::day_1::Display;
pub use tdd_kata::lcd_kata::day_1::Number::*;

pub use expectest::prelude::be_ok;

describe! lcd_tests {

    before_each {
        let mut display = Display::new();
    }

    it "should output nothing" {
        display.input("");

        expect!(display.output()).to(be_ok().value(vec![]));
    }

    it "should output one" {
        display.input("1");

        expect!(display.output()).to(be_ok().value(vec![One]));
    }

    it "should output more than one digit number" {
        display.input("1234567890");

        expect!(display.output()).to(be_ok().value(vec![One, Two, Three, Four, Five, Six, Seven, Eight, Nine, Zero]));
    }
}
