#![feature(const_fn,plugin)]
#![plugin(stainless)]

extern crate collections;

pub use self::collections::Map;

describe! map_tests {

    before_each {
        let mut map = Map::new();
    }

    it "should create a new empty map" {
        assert!(map.is_empty());
        assert_eq!(map.len(), 0);
    }

    it "should increase size when insert" {
        map.insert(1, 1);

        assert!(!map.is_empty());
    }

    it "should not increase size when insert the same key" {
        map.insert(1, 1);
        let old_size = map.len();
        map.insert(1, 10);

        assert_eq!(map.len(), old_size);
    }

    it "should contain inserted value" {
        map.insert(1, 1);

        assert!(map.contains(1));
    }

    it "should not contain not inserted value" {
        assert!(!map.contains(10));
    }

    it "should contain all inserted values" {
        map.insert(1, 1);
        map.insert(2, 1);
        map.insert(3, 1);
        map.insert(4, 1);

        assert!(map.contains(1));
        assert!(map.contains(2));
        assert!(map.contains(3));
        assert!(map.contains(4));
    }
}
