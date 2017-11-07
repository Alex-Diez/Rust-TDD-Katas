type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    item: T,
    next: Link<T>
}

impl <T> Node<T> {
    fn new(item: T, next: Link<T>) -> Self {
        Node { item, next }
    }
}

pub struct Stack<T> {
    head: Link<T>
}

impl <T> Stack<T> {
    pub fn new() -> Self {
        Stack { head: None }
    }

    pub fn push(&mut self, item: T) {
        let node = self.head.take();
        self.head = Some(Box::new(Node::new(item, node)));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            let node = *node;
            self.head = node.next;
            node.item
        })
    }
}

impl <T> IntoIterator for Stack<T> {
    type Item = T;
    type IntoIter = StackIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        StackIter { stack: self }
    }
}

pub struct StackIter<T> {
    stack: Stack<T>
}

impl <T> Iterator for StackIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

impl <T> AsRef<Stack<T>> for Stack<T> {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<'s, T> IntoIterator for &'s Stack<T> {
    type Item = &'s T;
    type IntoIter = StackIterRef<'s, T>;

    fn into_iter(self) -> Self::IntoIter {
        StackIterRef { node: self.head.as_ref().map(|node| &**node) }
    }
}

pub struct StackIterRef<'s, T: 's> {
    node: Option<&'s Node<T>>
}

impl<'s, T: 's> Iterator for StackIterRef<'s, T> {
    type Item = &'s T;

    fn next(&mut self) -> Option<Self::Item> {
        self.node.take().map(|node| {
            self.node = node.next.as_ref().map(|node| &**node);
            &node.item
        })
    }
}

impl <T> AsMut<Stack<T>> for Stack<T> {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl<'s, T> IntoIterator for &'s mut Stack<T> {
    type Item = &'s mut T;
    type IntoIter = StackIterRefMut<'s, T>;

    fn into_iter(self) -> Self::IntoIter {
        StackIterRefMut { node: self.head.as_mut().map(|node| &mut **node) }
    }
}

pub struct StackIterRefMut<'s, T: 's> {
    node: Option<&'s mut Node<T>>
}

impl <'s, T: 's> Iterator for StackIterRefMut<'s, T> {
    type Item = &'s mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.node.take().map(|node| {
            self.node = node.next.as_mut().map(|node| &mut **node);
            &mut node.item
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_many_items_pop_all() {
        let mut stack = Stack::new();

        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn iterate_over_stack_move() {
        let mut stack = Stack::new();

        stack.push(1);
        stack.push(2);
        stack.push(3);

        let mut iter = stack.into_iter();

        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iterate_over_stack_borrowing() {
        let mut stack = Stack::new();

        stack.push(1);
        stack.push(2);
        stack.push(3);

        let mut iter = stack.as_ref().into_iter();

        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iterate_over_stack_mut_borrowing() {
        let mut stack = Stack::new();

        stack.push(1);
        stack.push(2);
        stack.push(3);

        let mut iter = stack.as_mut().into_iter();

        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), None);
    }
}
