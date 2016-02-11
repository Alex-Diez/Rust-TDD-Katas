pub use tdd_kata::lcd_kata::day_3::Display;
pub use tdd_kata::lcd_kata::day_3::Number::*;

pub use expectest::prelude::be_some;

describe! lcd_tests {

    it "should create a new display" {
        Display::new();
    }

    it "should be nothing on empty display" {
        let dispaly = Display::new();

        expect!(dispaly.output()).to(be_some().value(vec![]));
    }

    it "should output input value" {
        let mut display = Display::new();

        display.input("1");

        expect!(display.output()).to(be_some().value(vec![One]));
    }

    it "should output all numbers" {
        let mut dispaly = Display::new();

        dispaly.input("1234567890");

        expect!(dispaly.output()).to(be_some().value(vec![One, Two, Three, Four, Five, Six, Seven, Eight, Nine, Zero]));
    }
}
