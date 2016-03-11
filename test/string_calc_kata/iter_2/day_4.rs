pub use tdd_kata::string_calc_kata::iter_2::day_4::evaluate;

pub use expectest::prelude::be_ok;

describe! string_calculator {

    it "should evaluate integer number" {
        expect!(evaluate("54357632471")).to(be_ok().value(54357632471.0));
    }

    it "shoould evaluate float number" {
        expect!(evaluate("4325546.1235454")).to(be_ok().value(4325546.1235454));
    }

    it "should add two numbers" {
        expect!(evaluate("32543.456+3254.231")).to(be_ok().value(35797.687));
    }

    it "should sub two numbers" {
        expect!(evaluate("3248.546-123.567")).to(be_ok().value(3124.979));
    }

    it "should mul two numbers" {
        expect!(evaluate("32254.546×15.5")).to(be_ok().value(499945.463));
    }

    it "should div two numbers" {
        expect!(evaluate("32543÷546.4")).to(be_ok().value(59.55893118594437));
    }

    it "should evaluate numbers of operations" {
        expect!(evaluate("3245+45.55+567.876-658561.54÷154+4325×25+3534")).to(be_ok().value(111241.05236363637));
    }
}
