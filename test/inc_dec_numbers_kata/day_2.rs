pub use tdd_kata::inc_dec_numbers_kata::day_2::total_inc_dec;

pub use expectest::prelude::be_equal_to;

describe! inc_dec_numbers_tests {

    it "should be 1 inc dec number of 0 power" {
        expect!(total_inc_dec(0)).to(be_equal_to(1));
    }

    it "should be 10 inc dec numbers of 1 power" {
        expect!(total_inc_dec(1)).to(be_equal_to(10));
    }

    it "should be 100 inc dec numbers of 2 power" {
        expect!(total_inc_dec(2)).to(be_equal_to(100));
    }
}
