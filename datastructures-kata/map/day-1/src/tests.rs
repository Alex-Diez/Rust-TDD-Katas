#![feature(const_fn,plugin)]
#![plugin(stainless)]

extern crate collections;

pub use collections::Map;

describe! map_tests {

    before_each {
        let mut map = Map::new();
    }

    it "should create a new empty map" {
        assert_eq!(map.size(), 0);
        assert!(map.is_empty());
    }

    it "should increase size when put into map" {
        let old_size = map.size();
        map.put(1, 1);

        assert_eq!(map.size(), old_size + 1);
        assert!(!map.is_empty());
    }

    it "should not increase size when put the same key into the map" {
        map.put(1, 1);
        let old_size = map.size();
        map.put(1, 2);

        assert_eq!(map.size(), old_size);
    }

    it "should decrease size when remove from map" {
        map.put(1, 1);
        let old_size = map.size();
        map.remove(1);

        assert_eq!(map.size(), old_size - 1);
    }

    it "should not decrease size when remove the same key from map" {
        map.put(1, 1);
        map.put(2, 1);
        map.remove(1);
        let old_size = map.size();
        map.remove(1);

        assert_eq!(map.size(), old_size);

    }
}
