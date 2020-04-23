use std::mem;
use std::iter::FromIterator;

type Link<K, V> = Option<Box<Node<K, V>>>;

struct Node<K: Ord, V> {
    key: K,
    value: V,
    left: Link<K, V>,
    right: Link<K, V>,
}

impl<K: Ord, V> Node<K, V> {
    fn new(key: K, value: V) -> Link<K, V> {
        Some(Box::new(Self { key, value, left: None, right: None }))
    }

    fn find(&self, key: &K) -> Option<&V> {
        if self.key == *key {
            Some(&self.value)
        } else if self.key < *key {
            self.left.as_ref().and_then(|left| left.find(key))
        } else {
            self.right.as_ref().and_then(|right| right.find(key))
        }
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.key < key {
            match self.left.as_mut() {
                None => {
                    self.left = Node::new(key, value);
                    None
                }
                Some(left) => left.insert(key, value)
            }
        } else if self.key > key {
            match self.right.as_mut() {
                None => {
                    self.right = Node::new(key, value);
                    None
                },
                Some(right) => right.insert(key, value)
            }
        } else {
            let mut temp = unsafe { mem::zeroed() };
            mem::swap(&mut temp, &mut self.value);
            self.value = value;
            Some(temp)
        }
    }

    fn pre_order<'n>(&'n self, mut items: Vec<&'n V>) -> Vec<&'n V> {
        items.push(&self.value);
        items = match self.right.as_ref() {
            Some(right) => right.pre_order(items),
            None => items
        };
        items = match self.left.as_ref() {
            Some(left) => left.pre_order(items),
            None => items
        };
        items
    }

    fn in_order<'n>(&'n self, mut items: Vec<&'n V>) -> Vec<&'n V> {
        items = match self.right.as_ref() {
            Some(right) => right.in_order(items),
            None => items
        };
        items.push(&self.value);
        items = match self.left.as_ref() {
            Some(left) => left.in_order(items),
            None => items
        };
        items
    }

    fn post_order<'n>(&'n self, mut items: Vec<&'n V>) -> Vec<&'n V> {
        items = match self.left.as_ref() {
            Some(left) => left.post_order(items),
            None => items
        };
        items = match self.right.as_ref() {
            Some(right) => right.post_order(items),
            None => items
        };
        items.push(&self.value);
        items
    }
}

#[derive(Default)]
pub struct Tree<K: Ord, V> {
    root: Link<K, V>
}

impl<K: Ord, V> Tree<K, V> {
    pub fn find(&self, key: &K) -> Option<&V> {
        self.root.as_ref().and_then(|root| root.find(key))
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        match self.root.as_mut() {
            None => {
                self.root = Node::new(key, value);
                None
            },
            Some(root) => root.insert(key, value)
        }
    }

    pub fn pre_order(&self) -> impl Iterator<Item=&V> {
        match self.root.as_ref() {
            None => TreeTraversal::from(vec![]),
            Some(root) => TreeTraversal::from(root.pre_order(vec![]))
        }
    }

    pub fn in_order(&self) -> impl Iterator<Item=&V> {
        match self.root.as_ref() {
            None => TreeTraversal::from(vec![]),
            Some(root) => TreeTraversal::from(root.in_order(vec![]))
        }
    }

    pub fn post_order(&self) -> impl Iterator<Item=&V> {
        match self.root.as_ref() {
            None => TreeTraversal::from(vec![]),
            Some(root) => TreeTraversal::from(root.post_order(vec![]))
        }
    }
}

struct TreeTraversal<'t, V> {
    items: Box<(dyn Iterator<Item=&'t V> + 't)>
}

impl<'t, V> From<Vec<&'t V>> for TreeTraversal<'t, V> {
    fn from(items: Vec<&'t V>) -> Self {
        Self { items: Box::new(items.into_iter()) }
    }
}

impl<'t, V> Iterator for TreeTraversal<'t, V> {
    type Item = &'t V;

    fn next(&mut self) -> Option<Self::Item> {
        self.items.next()
    }
}

impl<K: Ord + Default, V: Default> FromIterator<(K, V)> for Tree<K, V> {
    fn from_iter<II: IntoIterator<Item=(K, V)>>(iter: II) -> Self {
        let mut tree: Tree<K, V> = Tree { root: None };

        for (key, value) in iter {
            tree.insert(key, value);
        }

        tree
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_not_inserted_value() {
        let tree: Tree<i32, i32> = Tree::default();

        assert_eq!(tree.find(&1), None);
    }

    #[test]
    fn insert_and_find_single_level_tree() {
        let mut tree = Tree::default();

        tree.insert(1, 1);

        assert_eq!(tree.find(&1), Some(&1));
    }

    #[test]
    fn insert_and_find_two_levels_tree() {
        let mut tree = Tree::default();

        assert_eq!(tree.insert(2, 2), None);
        assert_eq!(tree.insert(1, 1), None);

        assert_eq!(tree.find(&1), Some(&1));
        assert_eq!(tree.find(&2), Some(&2));
    }

    #[test]
    fn insert_and_find_three_levels_tree() {
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
    fn update_inserted_values() {
        let mut tree = Tree::from_iter(vec![4, 2, 1, 3, 6, 5, 7].into_iter().enumerate());

        for (key, value) in vec![4, 2, 1, 3, 6, 5, 7].into_iter().enumerate() {
            assert_eq!(tree.insert(key, 2 * value), Some(value));
        }
    }

    #[test]
    fn pre_order() {
        let key_value_pairs = vec![4, 2, 1, 3, 6, 5, 7]
            .into_iter()
            .enumerate()
            .map(|(_index, value)| (value, value));
        let tree = Tree::from_iter(key_value_pairs);

        let mut pre_order = tree.pre_order();

        assert_eq!(pre_order.next(), Some(&4));
        assert_eq!(pre_order.next(), Some(&2));
        assert_eq!(pre_order.next(), Some(&1));
        assert_eq!(pre_order.next(), Some(&3));
        assert_eq!(pre_order.next(), Some(&6));
        assert_eq!(pre_order.next(), Some(&5));
        assert_eq!(pre_order.next(), Some(&7));
    }

    #[test]
    fn in_order() {
        let key_value_pairs = vec![4, 2, 1, 3, 6, 5, 7]
            .into_iter()
            .enumerate()
            .map(|(_index, value)| (value, value));
        let tree = Tree::from_iter(key_value_pairs);

        let mut in_order = tree.in_order();

        assert_eq!(in_order.next(), Some(&1));
        assert_eq!(in_order.next(), Some(&2));
        assert_eq!(in_order.next(), Some(&3));
        assert_eq!(in_order.next(), Some(&4));
        assert_eq!(in_order.next(), Some(&5));
        assert_eq!(in_order.next(), Some(&6));
        assert_eq!(in_order.next(), Some(&7));
    }

    #[test]
    fn post_order() {
        let key_value_pairs = vec![4, 2, 1, 3, 6, 5, 7]
            .into_iter()
            .enumerate()
            .map(|(_index, value)| (value, value));
        let tree = Tree::from_iter(key_value_pairs);

        let mut post_order = tree.post_order();

        assert_eq!(post_order.next(), Some(&7));
        assert_eq!(post_order.next(), Some(&5));
        assert_eq!(post_order.next(), Some(&6));
        assert_eq!(post_order.next(), Some(&3));
        assert_eq!(post_order.next(), Some(&1));
        assert_eq!(post_order.next(), Some(&2));
        assert_eq!(post_order.next(), Some(&4));
    }
}
