pub use tdd_kata::string_calc_kata::iter_2::day_1::evaluate;

pub use expectest::prelude::be_ok;

describe! calculator {

    it "should evaluate integer number" {
        expect!(evaluate("0")).to(be_ok().value(0.0));
    }

    it "should evaluate float number" {
        expect!(evaluate("0.0")).to(be_ok().value(0.0));
    }

    it "should evaluate sum of two numbers" {
        expect!(evaluate("1+2.0")).to(be_ok().value(3.0));
    }

    it "should evaluate subtraction" {
        expect!(evaluate("2.0-1")).to(be_ok().value(1.0));
    }

    it "should evaluate number of additions and subtractions" {
        expect!(evaluate("2.0+3-5.1+19.5-5.3-3.1+2+0.3-0.1")).to(be_ok().value(13.2));
    }
}
