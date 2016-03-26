#![allow(unused_variables)]

/*struct Node {
    key: isize,
    left: Option<Node>,
    right: Option<Node>
}

impl Node {

    fn new(key: isize) -> Node {
        Node {
            key: key,
            left: None,
            right: None
        }
    }
}*/

pub struct BinarySearchTree {
    len: usize,
//    root: Option<Node>
}

impl Default for BinarySearchTree {

    fn default() -> BinarySearchTree {
        BinarySearchTree {
            len: 0,
//            root: None
        }
    }
}

impl BinarySearchTree {

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn put(&mut self, key: isize, value: isize) {
        /*match self.root {
            Some(node) => {
                loop {
                    while node.key < key {
                        node = match node.left {

                        }
                    }
                }
            },
            None => {
                self.root = Some(Node::new(key));
                self.len += 1;
            },
        }*/
    }

/*    fn put_node(&mut self, key: isize, node: &Option<Node>) {
        match *node {
            Some(node) => {
                if node.key < key {
                    match node.left {
                        Some(left) => self.put_node(key, &node.left),
                        None => node.left = Some(Node::new(key)),
                    }
                }
                if node.key > key {
                    match node.right {
                        Some(right) => self.put_node(key, &node.right),
                        None => node.right = Some(Node::new(key)),
                    }
                }
            },
            None => {},
        }
    }*/

    pub fn contains(&self, key: isize) -> bool {
        // self.contains_node(key, &(self.root))
        false
    }

/*    fn contains_node(&self, key: isize, node: &Option<Node>) -> bool {
        match *node {
            Some(node) => {
                if node.key < key {
                    match node.left {
                        Some(left) => self.contains_node(key, &node.left),
                        None => false,
                    }
                }
                else if node.key > key {
                    match node.right {
                        Some(right) => self.contains_node(key, &node.right),
                        None => false,
                    }
                }
                else {
                    node.key == key
                }
            },
            None => false,
        }
        false
    }*/
}
