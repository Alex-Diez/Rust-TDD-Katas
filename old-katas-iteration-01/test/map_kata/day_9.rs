pub use tdd_kata::map_kata::day_9::Map;

pub use expectest::prelude::{be_equal_to, be_true, be_false, be_some};

describe! map_tests {

    before_each {
        let mut map = Map::new();
    }

    it "should create a new map" {
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
        expect!(map.contains(10)).to(be_false());
    }

    it "should contain all inserted keys" {
        map.insert(1, 1);
        map.insert(2, 1);
        map.insert(3, 1);
        map.insert(4, 1);
        map.insert(5, 1);

        expect!(map.contains(1)).to(be_true());
        expect!(map.contains(2)).to(be_true());
        expect!(map.contains(3)).to(be_true());
        expect!(map.contains(4)).to(be_true());
        expect!(map.contains(5)).to(be_true());
    }

    it "should retrieve inserted value" {
        map.insert(1, 1);

        expect!(map.get(1)).to(be_some().value(1));
    }

    it "should retrieve new inserted value" {
        map.insert(1, 1);

        map.insert(1, 10);

        expect!(map.get(1)).to(be_some().value(10));
    }
}
