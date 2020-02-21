pub use tdd_kata::string_calc_kata::iter_2::day_6::evaluate;

pub use expectest::prelude::be_ok;

describe! string_calculator {

    it "should evaluate float number" {
        expect!(evaluate("4354.5405478")).to(be_ok().value(4354.5405478));
    }

    it "should evaluate add operation" {
        expect!(evaluate("43654.45+32432.42")).to(be_ok().value(76086.87));
    }

    it "should evaluate sub operation" {
        expect!(evaluate("43654.456-312.547")).to(be_ok().value(43341.909));
    }

    it "should evaluate mul operation" {
        expect!(evaluate("435.3×353.2")).to(be_ok().value(153747.96));
    }

    it "should evaluate div operation" {
        expect!(evaluate("435.678÷213.2")).to(be_ok().value(2.043517823639775));
    }

    it "should evaluate number of operaions" {
        expect!(evaluate("435+657.6×2-8769.25÷23÷2+65467.25-15.48+15×15.25")).to(be_ok().value(67240.08413043479));
    }

    it "should evaluate expression with parenthesis" {
        expect!(evaluate("345.678+(456.2-3243.2÷24.2)+15.25-48.5×(32465.34-(3243.14+543)×2)-45.57")).to(be_ok().value(-1206675.8685289258));
    }
}
