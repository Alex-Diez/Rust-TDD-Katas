pub use tdd_kata::string_calc_kata::iter_2::day_1::evaluate;

pub use expectest::prelude::be_ok;

describe! calculator {

    it "should evaluate one digit number" {
        expect!(evaluate("1")).to(be_ok().value(1.0));
    }

    it "should evaluate more than one digits number" {
        expect!(evaluate("12345")).to(be_ok().value(12345.0));
    }

    it "should evaluate float number" {
        expect!(evaluate("213.4576")).to(be_ok().value(213.4576));
    }
}
