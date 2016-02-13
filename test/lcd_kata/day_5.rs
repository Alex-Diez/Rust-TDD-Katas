pub use tdd_kata::lcd_kata::day_5::Display;
pub use tdd_kata::lcd_kata::day_5::Number::{One, Two, Three, Four, Five, Six, Seven, Eight, Nine, Zero};
pub use tdd_kata::lcd_kata::day_5::Data::{NotANumber, Output};

pub use expectest::prelude::be_equal_to;

describe! lcd_tests {

    before_each {
        let mut display = Display::new();
    }

    it "should output nothing on new display " {
        expect!(display.output()).to(be_equal_to(Output(vec![])));
    }

    it "should output nothing with empty input" {
        display.input("");

        expect!(display.output()).to(be_equal_to(Output(vec![])));
    }

    it "should output one" {
        display.input("1");

        expect!(display.output()).to(be_equal_to(Output(vec![One])));
    }

    it "should output all numbers" {
        display.input("1234567890");

        expect!(display.output()).to(be_equal_to(Output(vec![One, Two, Three, Four, Five, Six, Seven, Eight, Nine, Zero])));
    }

    it "should show error when input is not a number" {
        display.input("abc");

        expect!(display.output()).to(be_equal_to(NotANumber));
    }
}
