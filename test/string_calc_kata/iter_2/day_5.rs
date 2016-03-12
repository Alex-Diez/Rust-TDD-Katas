pub use tdd_kata::string_calc_kata::iter_2::day_5::Calculator;

pub use expectest::prelude::be_ok;

describe! string_calculator {

    before_each {
        let calc = Calculator::new();
    }

    it "should evaluate number" {
        expect!(calc.evaluate("4350.264")).to(be_ok().value(4350.264));
    }

    it "should evaluate add operation" {
        expect!(calc.evaluate("34543.767+324.654")).to(be_ok().value(34868.421));
    }

    it "should evaluate sub operation" {
        expect!(calc.evaluate("1345.56-345.34")).to(be_ok().value(1000.22));
    }

    it "should evaluate mul operation" {
        expect!(calc.evaluate("3254345.435×34.3")).to(be_ok().value(111624048.4205));
    }

    it "should evaluate div operation" {
        expect!(calc.evaluate("5436.321÷435.3")).to(be_ok().value(12.48867677463818));
    }

    it "should evaluate number of operations" {
        expect!(calc.evaluate("54.546+123.435×53.2-546.248×2+16576.659÷58.2-165.54")).to(be_ok().value(5648.074319587629));
    }

    it "should evaluate expression with parenthesis" {
        expect!(calc.evaluate("435+(345.6-43.6×2)-3244.2×(43543.435-45682.54)+4353425.58÷(324+(3455-2321)-1240)")).to(be_ok().value(6960347.683110102));
    }
}
