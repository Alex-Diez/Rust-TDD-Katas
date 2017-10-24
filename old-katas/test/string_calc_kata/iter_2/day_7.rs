pub use tdd_kata::string_calc_kata::iter_2::day_7::evaluate;

pub use expectest::prelude::be_ok;

describe! string_calculator {

    it "should evaluate float number" {
        expect!(evaluate("32435.246")).to(be_ok().value(32435.246));
    }

    it "should evaluate add operation" {
        expect!(evaluate("35435.657+213.546")).to(be_ok().value(35649.203));
    }

    it "should evaluate sub operation" {
        expect!(evaluate("3465.6757-324.2346")).to(be_ok().value(3141.4411));
    }

    it "should evaluate mul operation" {
        expect!(evaluate("354.76×25.2")).to(be_ok().value(8939.952));
    }

    it "should evaluate div operation" {
        expect!(evaluate("3254.546÷32.32")).to(be_ok().value(100.69758663366336));
    }

    it "should evaluate sequnce of operations" {
        expect!(evaluate("3254+324×23-461.125×2+4.248÷23-461×1.25+48")).to(be_ok().value(9255.684695652173));
    }

    it "should evaluate expression with parenthesis" {
        expect!(evaluate("3425+214+(213.3-22.4×12)-3254×(234.2+32.2)+54÷2")).to(be_ok().value(-863255.1));
    }
}
