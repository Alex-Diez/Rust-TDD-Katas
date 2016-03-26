pub struct BinarySearchTree;

impl Default for BinarySearchTree {

    fn default() -> BinarySearchTree {
        BinarySearchTree
    }
}

impl BinarySearchTree {

    pub fn len(&self) -> usize {
        0
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
