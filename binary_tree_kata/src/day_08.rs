use std::cmp::Ordering;
use std::iter::FromIterator;
use core::mem;

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
        match key.cmp(&self.key) {
            Ordering::Equal => Some(&self.value),
            Ordering::Less => self.left.as_ref().and_then(|left| left.find(key)),
            Ordering::Greater => self.right.as_ref().and_then(|right| right.find(key)),
        }
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        match key.cmp(&self.key) {
            Ordering::Equal => {
                let mut temp = unsafe { mem::zeroed() };
                mem::swap(&mut temp, &mut self.value);
                self.value = value;
                Some(temp)
            }
            Ordering::Less => {
                match self.left.as_mut() {
                    Some(left) => left.insert(key, value),
                    None => {
                        self.left = Node::new(key, value);
                        None
                    }
                }
            }
            Ordering::Greater => {
                match self.right.as_mut() {
                    Some(right) => right.insert(key, value),
                    None => {
                        self.right = Node::new(key, value);
                        None
                    }
                }
            }
        }
    }

    fn pre_order<'t>(&'t self, mut items: Vec<&'t V>) -> Vec<&'t V> {
        items.push(&self.value);
        items = match self.left.as_ref() {
            Some(left) => left.pre_order(items),
            None => items
        };
        items = match self.right.as_ref() {
            Some(right) => right.pre_order(items),
            None => items
        };
        items
    }

    fn in_order<'t>(&'t self, mut items: Vec<&'t V>) -> Vec<&'t V> {
        items = match self.left.as_ref() {
            Some(left) => left.in_order(items),
            None => items
        };
        items.push(&self.value);
        items = match self.right.as_ref() {
            Some(right) => right.in_order(items),
            None => items
        };
        items
    }

    fn post_order<'t>(&'t self, mut items: Vec<&'t V>) -> Vec<&'t V> {
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

    fn pre_order_mut<'t>(&'t mut self, mut items: Vec<&'t mut V>) -> Vec<&'t mut V> {
        items.push(&mut self.value);
        items = match self.left.as_mut() {
            Some(left) => left.pre_order_mut(items),
            None => items
        };
        items = match self.right.as_mut() {
            Some(right) => right.pre_order_mut(items),
            None => items
        };
        items
    }

    fn in_order_mut<'t>(&'t mut self, mut items: Vec<&'t mut V>) -> Vec<&'t mut V> {
        items = match self.left.as_mut() {
            Some(left) => left.in_order_mut(items),
            None => items
        };
        items.push(&mut self.value);
        items = match self.right.as_mut() {
            Some(right) => right.in_order_mut(items),
            None => items
        };
        items
    }

    fn post_order_mut<'t>(&'t mut self, mut items: Vec<&'t mut V>) -> Vec<&'t mut V> {
        items = match self.left.as_mut() {
            Some(left) => left.post_order_mut(items),
            None => items
        };
        items = match self.right.as_mut() {
            Some(right) => right.post_order_mut(items),
            None => items
        };
        items.push(&mut self.value);
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
            Some(root) => root.insert(key, value),
            None => {
                self.root = Node::new(key, value);
                None
            }
        }
    }

    pub fn pre_order(&self) -> impl Iterator<Item=&V> {
        TreeTraversal::from(
            self.root.as_ref()
                .map(|root| root.pre_order(vec![]))
                .unwrap_or_default()
        )
    }

    pub fn in_order(&self) -> impl Iterator<Item=&V> {
        TreeTraversal::from(
            self.root.as_ref()
                .map(|root| root.in_order(vec![]))
                .unwrap_or_default()
        )
    }

    pub fn post_order(&self) -> impl Iterator<Item=&V> {
        TreeTraversal::from(
            self.root.as_ref()
                .map(|root| root.post_order(vec![]))
                .unwrap_or_default()
        )
    }

    pub fn pre_order_mut(&mut self) -> impl Iterator<Item=&mut V> {
        TreeTraversalMut::from(
            self.root.as_mut()
                .map(|root| root.pre_order_mut(vec![]))
                .unwrap_or_default()
        )
    }

    pub fn in_order_mut(&mut self) -> impl Iterator<Item=&mut V> {
        TreeTraversalMut::from(
            self.root.as_mut()
                .map(|root| root.in_order_mut(vec![]))
                .unwrap_or_default()
        )
    }

    pub fn post_order_mut(&mut self) -> impl Iterator<Item=&mut V> {
        TreeTraversalMut::from(
            self.root.as_mut()
                .map(|root| root.post_order_mut(vec![]))
                .unwrap_or_default()
        )
    }
}

impl<K: Ord + Default, V: Default> FromIterator<(K, V)> for Tree<K, V> {
    fn from_iter<II: IntoIterator<Item=(K, V)>>(iter: II) -> Self {
        let mut tree = Tree::default();

        for (key, value) in iter {
            tree.insert(key, value);
        }

        tree
    }
}

struct TreeTraversal<'t, V> {
    items: Box<dyn Iterator<Item=&'t V> + 't>
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

struct TreeTraversalMut<'t, V> {
    items: Box<dyn Iterator<Item=&'t mut V> + 't>
}

impl<'t, V> From<Vec<&'t mut V>> for TreeTraversalMut<'t, V> {
    fn from(items: Vec<&'t mut V>) -> Self {
        Self { items: Box::new(items.into_iter()) }
    }
}

impl<'t, V> Iterator for TreeTraversalMut<'t, V> {
    type Item = &'t mut V;

    fn next(&mut self) -> Option<Self::Item> {
        self.items.next()
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
    fn insert_and_find_single_value() {
        let mut tree = Tree::default();

        assert_eq!(tree.insert(1, 1), None);

        assert_eq!(tree.find(&1), Some(&1));
    }

    #[test]
    fn insert_and_find_many_values() {
        let mut tree = Tree::default();

        assert_eq!(tree.insert(2, 2), None);
        assert_eq!(tree.insert(1, 1), None);
        assert_eq!(tree.insert(3, 3), None);

        assert_eq!(tree.find(&1), Some(&1));
        assert_eq!(tree.find(&2), Some(&2));
        assert_eq!(tree.find(&3), Some(&3));
    }

    #[test]
    fn insert_and_find_tree_level_tree() {
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

    fn pairs() -> impl Iterator<Item=(i32, i32)> {
        vec![4, 2, 1, 3, 6, 5, 7]
            .into_iter()
            .map(|value| (value, value))
    }

    #[test]
    fn replace_values() {
        let mut tree = Tree::from_iter(pairs());

        for (key, value) in pairs() {
            assert_eq!(tree.insert(key, value * 2), Some(value));
        }

        for (key, value) in pairs() {
            assert_eq!(tree.insert(key, value * 3), Some(value * 2));
        }
    }

    #[test]
    fn pre_order() {
        let tree = Tree::from_iter(pairs());

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
        let tree = Tree::from_iter(pairs());

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
        let tree = Tree::from_iter(pairs());

        let mut post_order = tree.post_order();

        assert_eq!(post_order.next(), Some(&1));
        assert_eq!(post_order.next(), Some(&3));
        assert_eq!(post_order.next(), Some(&2));
        assert_eq!(post_order.next(), Some(&5));
        assert_eq!(post_order.next(), Some(&7));
        assert_eq!(post_order.next(), Some(&6));
        assert_eq!(post_order.next(), Some(&4));
    }

    #[test]
    fn pre_order_mut() {
        let mut tree = Tree::from_iter(pairs());

        let mut pre_order = tree.pre_order_mut();

        assert_eq!(pre_order.next(), Some(&mut 4));
        assert_eq!(pre_order.next(), Some(&mut 2));
        assert_eq!(pre_order.next(), Some(&mut 1));
        assert_eq!(pre_order.next(), Some(&mut 3));
        assert_eq!(pre_order.next(), Some(&mut 6));
        assert_eq!(pre_order.next(), Some(&mut 5));
        assert_eq!(pre_order.next(), Some(&mut 7));
    }

    #[test]
    fn in_order_mut() {
        let mut tree = Tree::from_iter(pairs());

        let mut in_order = tree.in_order_mut();

        assert_eq!(in_order.next(), Some(&mut 1));
        assert_eq!(in_order.next(), Some(&mut 2));
        assert_eq!(in_order.next(), Some(&mut 3));
        assert_eq!(in_order.next(), Some(&mut 4));
        assert_eq!(in_order.next(), Some(&mut 5));
        assert_eq!(in_order.next(), Some(&mut 6));
        assert_eq!(in_order.next(), Some(&mut 7));
    }

    #[test]
    fn post_order_mut() {
        let mut tree = Tree::from_iter(pairs());

        let mut post_order = tree.post_order_mut();

        assert_eq!(post_order.next(), Some(&mut 1));
        assert_eq!(post_order.next(), Some(&mut 3));
        assert_eq!(post_order.next(), Some(&mut 2));
        assert_eq!(post_order.next(), Some(&mut 5));
        assert_eq!(post_order.next(), Some(&mut 7));
        assert_eq!(post_order.next(), Some(&mut 6));
        assert_eq!(post_order.next(), Some(&mut 4));
    }
}
