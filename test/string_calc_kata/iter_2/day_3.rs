pub use tdd_kata::string_calc_kata::iter_2::day_3::evaluate;

pub use expectest::prelude::be_ok;

describe! string_calculator {

    it "should evaluate integer number" {
        expect!(evaluate("324340")).to(be_ok().value(324340.0));
    }

    it "should evaluate float number" {
        expect!(evaluate("2345054.05434")).to(be_ok().value(2345054.05434));
    }

    it "should evaluate addition of two numbers" {
        expect!(evaluate("32.43+123.435")).to(be_ok().value(155.865));
    }

    it "should evaluate subtraction of two numbers" {
        expect!(evaluate("15461.255-5484.2")).to(be_ok().value(9977.055));
    }

    it "should evaluate number of add and sub operation" {
        expect!(evaluate("214325.24-23423.657+23432.25-854+5624.1")).to(be_ok().value(219103.933));
    }

    it "should evaluate multiplication of two numbers" {
        expect!(evaluate("2435×2654")).to(be_ok().value(6462490.0));
    }

    it "should evaluate division of two numbers" {
        expect!(evaluate("4354÷2")).to(be_ok().value(2177.0));
    }
}
