struct Node<T> {
    val: T,
    next: Link<T>
}

type Link<T> = Option<Box<Node<T>>>;

pub struct Stack<T> {
    head: Link<T>
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { head: None }
    }

    pub fn push(&mut self, val: T) {
        let node = Box::new(Node {
            val: val,
            next: self.head.take()
        });

        self.head = Some(node)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            let node = *node;
            self.head = node.next;
            node.val
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.val)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.val)
    }
}

impl<T> AsMut<Stack<T>> for Stack<T> {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl<'mi, T> IntoIterator for &'mi mut Stack<T> {
    type Item = &'mi mut T;
    type IntoIter = StackMutRefIterator<'mi, T>;

    fn into_iter(self) -> Self::IntoIter {
        StackMutRefIterator { next: self.head.as_mut().map(|node| &mut **node) }
    }
}

pub struct StackMutRefIterator<'mi, T: 'mi> {
    next: Option<&'mi mut Node<T>>
}

impl<'mi, T> Iterator for StackMutRefIterator<'mi, T> {
    type Item = &'mi mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_mut().map(|node| &mut **node);
            &mut node.val
        })
    }
}

impl<T> AsRef<Stack<T>> for Stack<T> {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<'i, T> IntoIterator for &'i Stack<T> {
    type Item = &'i T;
    type IntoIter = StackRefIterator<'i, T>;

    fn into_iter(self) -> Self::IntoIter {
        StackRefIterator { next: self.head.as_ref().map(|node| &**node) }
    }
}

pub struct StackRefIterator<'i, T: 'i> {
    next: Option<&'i Node<T>>
}

impl<'i, T> Iterator for StackRefIterator<'i, T> {
    type Item = &'i T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.val
        })
    }
}

impl<T> IntoIterator for Stack<T> {
    type Item = T;
    type IntoIter = StackIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        StackIterator { stack: self }
    }
}

pub struct StackIterator<T> {
    stack: Stack<T>
}

impl<T> Iterator for StackIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn push_many(stack: &mut Stack<i32>, size: i32) {
        for i in 1..size + 1 {
            stack.push(i);
        }
    }

    #[test]
    fn creates_an_empty_stack() {
        let mut stack: Stack<i32> = Stack::new();

        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn pushes_an_element_on_a_stack() {
        let mut stack = Stack::new();

        push_many(&mut stack, 1);

        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn pushes_three_elements_on_a_stack() {
        let mut stack = Stack::new();

        push_many(&mut stack, 3);

        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn peeks_stack_head() {
        let mut stack = Stack::new();

        push_many(&mut stack, 2);

        assert_eq!(stack.peek(), Some(&2));
        assert_eq!(stack.peek(), Some(&2));
    }

    #[test]
    fn peeks_mut_stack_head() {
        let mut stack = Stack::new();

        push_many(&mut stack, 2);

        assert_eq!(stack.peek_mut(), Some(&mut 2));
        assert_eq!(stack.peek_mut(), Some(&mut 2));
    }

    #[test]
    fn value_iterator_over_an_empty_stack() {
        let stack: Stack<i32> = Stack::new();

        let mut iterator = stack.into_iter();

        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn value_iterator_over_a_nonempty_stack() {
        let mut stack = Stack::new();

        push_many(&mut stack, 3);

        let mut iterator = stack.into_iter();

        assert_eq!(iterator.next(), Some(3));
        assert_eq!(iterator.next(), Some(2));
        assert_eq!(iterator.next(), Some(1));
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn reference_iterator_over_a_nonempty_stack() {
        let mut stack = Stack::new();

        push_many(&mut stack, 3);

        let mut iterator = stack.as_ref().into_iter();

        assert_eq!(iterator.next(), Some(&3));
        assert_eq!(iterator.next(), Some(&2));
        assert_eq!(iterator.next(), Some(&1));
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn mutable_reference_iterator_over_a_nonempty_stack() {
        let mut stack = Stack::new();

        push_many(&mut stack, 3);

        let mut iterator = stack.as_mut().into_iter();

        assert_eq!(iterator.next(), Some(&mut 3));
        assert_eq!(iterator.next(), Some(&mut 2));
        assert_eq!(iterator.next(), Some(&mut 1));
        assert_eq!(iterator.next(), None);
    }
}