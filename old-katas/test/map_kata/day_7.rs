pub use tdd_kata::map_kata::day_7::Map;
pub use expectest::prelude::{be_true, be_false, be_some, be_equal_to};

describe! map_tests {

    before_each {
        let mut map = Map::new();
    }

    it "should create a new empty map" {
        expect!(map.len()).to(be_equal_to(0));
        expect!(map.is_empty()).to(be_true());
    }

    it "should increase size when insert" {
        map.insert(1, 1);

        expect!(map.is_empty()).to(be_false());
    }

    it "should not increase size when insert the same key" {
        map.insert(1, 1);
        let old_size = map.len();

        map.insert(1, 10);

        expect!(map.len()).to(be_equal_to(old_size));
    }

    it "should contain inserted key" {
        map.insert(1, 1);

        expect!(map.contains(1)).to(be_true());
    }

    it "should not contain not inserted key" {
        expect!(map.contains(20)).to(be_false());
    }

    it "should contain all inserted keys" {
        map.insert(1, 10);
        map.insert(2, 10);
        map.insert(3, 10);
        map.insert(4, 10);
        map.insert(5, 10);

        expect!(map.contains(1)).to(be_true());
        expect!(map.contains(2)).to(be_true());
        expect!(map.contains(3)).to(be_true());
        expect!(map.contains(4)).to(be_true());
        expect!(map.contains(5)).to(be_true());
    }

    it "should retrieve inserted value" {
        map.insert(1, 10);

        expect!(map.get(1)).to(be_some().value(10));
    }

    it "should change value for the next insert" {
        map.insert(1, 1);
        map.insert(1, 10);

        expect!(map.get(1)).to(be_some().value(10));
    }
}
