use std::ptr::NonNull;
use std::cmp::Ordering;

type Link<K, V> = Option<NonNull<Node<K, V>>>;

struct Node<K: Ord, V: Copy> {
    key: K,
    value: V,
    left: Link<K, V>,
    right: Link<K, V>,
}

impl<K: Ord, V: Copy> Node<K, V> {
    fn new(key: K, value: V) -> Box<Self> {
        Box::new(Self { key, value, left: None, right: None })
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        match key.cmp(&self.key) {
            Ordering::Less => {
                match self.left {
                    Some(mut left) => unsafe { left.as_mut().insert(key, value) },
                    None => {
                        self.left = NonNull::new(Box::into_raw(Node::new(key, value)));
                        None
                    }
                }
            },
            Ordering::Equal => {
                let prev = self.value;
                self.value = value;
                Some(prev)
            }
            Ordering::Greater => {
                match self.right {
                    Some(mut right) => unsafe { right.as_mut().insert(key, value) },
                    None => {
                        self.right = NonNull::new(Box::into_raw(Node::new(key, value)));
                        None
                    }
                }
            }
        }
    }

    fn search(&self, key: &K) -> Option<&V> {
        match key.cmp(&self.key) {
            Ordering::Less => self.left.as_ref().and_then(|left| search(left, key)),
            Ordering::Equal => Some(&self.value),
            Ordering::Greater => self.right.as_ref().and_then(|right| search(right, key))
        }
    }
}

#[allow(clippy::trivially_copy_pass_by_ref)]
fn search<'n, K: Ord, V: Copy>(node: &'n NonNull<Node<K, V>>, key: &K) -> Option<&'n V> {
    unsafe { node.as_ref().search(key) }
}

#[derive(Default)]
pub struct Tree<K: Ord, V: Copy> {
    root: Link<K, V>
}

impl<K: Ord, V: Copy> Tree<K, V> {
    pub fn find(&self, key: &K) -> Option<&V> {
        self.root.as_ref().and_then(|root| search(root, key) )
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        unsafe {
            match self.root {
                None => {
                    self.root = NonNull::new(Box::into_raw(Node::new(key, value)));
                    None
                },
                Some(mut root) => root.as_mut().insert(key, value)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_item_in_empty_tree() {
        let tree: Tree<i32, i32> = Tree::default();

        assert_eq!(tree.find(&1), None);
    }

    #[test]
    fn find_inserted_value() {
        let mut tree = Tree::default();

        assert_eq!(tree.insert(1, 1), None);

        assert_eq!(tree.find(&1), Some(&1));
    }

    #[test]
    fn two_levels_tree_insert_and_find_left() {
        let mut tree = Tree::default();

        assert_eq!(tree.insert(2, 2), None);
        assert_eq!(tree.insert(1, 1), None);

        assert_eq!(tree.find(&1), Some(&1));
        assert_eq!(tree.find(&2), Some(&2));
    }

    #[test]
    fn two_levels_tree_insert_and_find_right() {
        let mut tree = Tree::default();

        assert_eq!(tree.insert(2, 2), None);
        assert_eq!(tree.insert(3, 3), None);

        assert_eq!(tree.find(&2), Some(&2));
        assert_eq!(tree.find(&3), Some(&3));
    }

    #[test]
    fn three_levels_tree() {
        let mut tree = Tree::default();

        assert_eq!(tree.insert(4, 4), None);
        assert_eq!(tree.insert(2, 2), None);
        assert_eq!(tree.insert(1, 1), None);
        assert_eq!(tree.insert(3, 3), None);
        assert_eq!(tree.insert(6, 6), None);
        assert_eq!(tree.insert(5, 5), None);
        assert_eq!(tree.insert(7, 7), None);

        assert_eq!(tree.find(&1), Some(&1));
        assert_eq!(tree.find(&2), Some(&2));
        assert_eq!(tree.find(&3), Some(&3));
        assert_eq!(tree.find(&4), Some(&4));
        assert_eq!(tree.find(&5), Some(&5));
        assert_eq!(tree.find(&6), Some(&6));
        assert_eq!(tree.find(&7), Some(&7));
    }

    #[test]
    fn replace_values() {
        let mut tree = Tree::default();

        for value in vec![4, 2, 1, 3, 6, 5, 7] {
            assert_eq!(tree.insert(value, value), None);
        }

        for value in vec![4, 2, 1, 3, 6, 5, 7] {
            assert_eq!(tree.insert(value, 2 * value), Some(value));
        }
    }
}
