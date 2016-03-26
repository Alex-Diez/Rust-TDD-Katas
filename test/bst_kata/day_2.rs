pub use tdd_kata::bst_kata::day_2::BinarySearchTree;

pub use expectest::prelude::{be_equal_to, be_true};

describe! search_tree {

    it "should create an empty binary search tree" {
        let tree = BinarySearchTree::default();

        let len = tree.len();
        let is_empty = tree.is_empty();

        expect!(len).to(be_equal_to(0));
        expect!(is_empty).to(be_true());
    }
}