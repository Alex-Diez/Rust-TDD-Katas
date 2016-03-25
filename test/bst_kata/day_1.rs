pub use tdd_kata::bst_kata::day_1::BinarySearchTree;

pub use expectest::prelude::{be_equal_to, be_true, be_false};

describe! search_tree {

    before_each {
        let mut tree = BinarySearchTree::default();
    }

    it "should create a new empty tree" {
        let len = tree.len();
        let is_empty = tree.is_empty();
        expect!(len).to(be_equal_to(0));
        expect!(is_empty).to(be_true());
    }

    it "should increase size when put" {
        tree.put(1, 1);
        expect!(tree.len()).to(be_equal_to(1));
    }

    it "should not increase size when put the same key" {
        tree.put(1, 1);
        tree.put(1, 2);
        expect!(tree.len()).to(be_equal_to(1));
    }

    it "should contain put key" {
        tree.put(1, 1);
        expect!(tree.contains(1)).to(be_true());
    }

    it "should not contain not put key" {
        expect!(tree.contains(2)).to(be_false());
    }

    it "should contain all put keys" {
        tree.put(1, 1);
        tree.put(2, 2);
        tree.put(3, 3);
        let contains = tree.contains(1);
        expect!(contains).to(be_true());
        let contains = tree.contains(2);
        expect!(contains).to(be_true());
        let contains = tree.contains(3);
        expect!(contains).to(be_true());
    }
}
