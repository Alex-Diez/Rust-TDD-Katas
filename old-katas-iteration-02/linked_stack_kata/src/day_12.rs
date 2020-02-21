type Link = Option<Box<Node>>;

struct Node {
    item: i32,
    next: Link
}

impl Node {
    fn new(item: i32, next: Link) -> Link {
        Some(Box::new(Node { item, next }))
    }
}

pub struct Stack {
    head: Link
}

impl Stack {
    pub fn pop(&mut self) -> Option<i32> {
        self.head.take().map(|node| {
            let node = *node;
            self.head = node.next;
            node.item
        })
    }

    pub fn push(&mut self, item: i32) {
        let old_head = self.head.take();
        self.head = Node::new(item, old_head);
    }
}

impl Default for Stack {
    fn default() -> Self {
        Stack { head: None }
    }
}

impl IntoIterator for Stack {
    type Item = i32;
    type IntoIter = StackIter;

    fn into_iter(self) -> Self::IntoIter {
        StackIter { stack: self }
    }
}

pub struct StackIter {
    stack: Stack
}

impl Iterator for StackIter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

impl AsRef<Stack> for Stack {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<'s> IntoIterator for &'s Stack {
    type Item = &'s i32;
    type IntoIter = StackIterRef<'s>;

    fn into_iter(self) -> Self::IntoIter {
        StackIterRef { node: self.head.as_ref().map(|node| &**node) }
    }
}

pub struct StackIterRef<'s> {
    node: Option<&'s Node>
}

impl<'s> Iterator for StackIterRef<'s> {
    type Item = &'s i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.node.take().map(|node| {
            self.node = node.next.as_ref().map(|node| &**node);
            &node.item
        })
    }
}

impl AsMut<Stack> for Stack {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl<'s> IntoIterator for &'s mut Stack {
    type Item = &'s mut i32;
    type IntoIter = StackIterRefMut<'s>;

    fn into_iter(self) -> Self::IntoIter {
        StackIterRefMut { node: self.head.as_mut().map(|node| &mut **node) }
    }
}

pub struct StackIterRefMut<'s> {
    node: Option<&'s mut Node>
}

impl<'s> Iterator for StackIterRefMut<'s> {
    type Item = &'s mut i32;

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

    fn fill_the_stack(stack: &mut Stack) {
        stack.push(1);
        stack.push(2);
        stack.push(3);
    }

    #[test]
    fn push_pop_many_items() {
        let mut stack = Stack::default();

        fill_the_stack(&mut stack);

        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn iterate_over_stack_move() {
        let mut stack = Stack::default();

        fill_the_stack(&mut stack);

        let mut iter = stack.into_iter();

        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iterate_over_stack_borrow() {
        let mut stack = Stack::default();

        fill_the_stack(&mut stack);

        let mut iter = stack.as_ref().into_iter();

        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iterate_over_stack_borrow_mut() {
        let mut stack = Stack::default();

        fill_the_stack(&mut stack);

        let mut iter = stack.as_mut().into_iter();

        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), None);
    }
}
