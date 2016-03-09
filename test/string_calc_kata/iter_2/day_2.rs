pub use tdd_kata::string_calc_kata::iter_2::day_2::evaluate;

pub use expectest::prelude::{be_ok, be_close_to};

describe! string_calculator {

    it "should evaluate int number" {
        expect!(evaluate("435546025041570")).to(be_ok().value(435546025041570.0));
    }

    it "should evaluate float number" {
        expect!(evaluate("4365477658.30244565")).to(be_ok().value(4365477658.30244565));
    }

    it "should evaluate addition of two numbers" {
        expect!(evaluate("23545.345+5468.8654")).to(be_ok().value(29014.2104));
    }

    it "should evaluate subtraction of two numbers" {
        expect!(evaluate("23545.34-5468.86")).to(be_ok().value(18076.48));
    }

    it "should evaluate number of add and sub operations" {
        expect!(evaluate("23545.34-5468.86+58982.25+542.26-7461.11-83.29")).to(be_ok().value(70056.59));
    }

    it "should evaluate multiplications" {
        expect!(evaluate("23545.34×3-5468.86×2")).to(be_ok().value(59698.3));
    }

    it "should evaluate divisions" {
        expect!(evaluate("23545.34÷2-5468.86÷3")).to(be_ok().value(9949.716666666667));
    }
}
