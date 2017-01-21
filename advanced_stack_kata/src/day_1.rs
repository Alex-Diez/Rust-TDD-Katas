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

        self.head = Some(node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|boxed| {
            let node = *boxed;
            self.head = node.next;
            node.val
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.val
        })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.val
        })
    }
}

impl<'i, T> IntoIterator for &'i Stack<T> {
    type Item = &'i T;
    type IntoIter = StackIterator<'i, T>;

    fn into_iter(self) -> Self::IntoIter {
        StackIterator { next: self.head.as_ref().map(|node| { &**node }) }
    }
}

pub struct StackIterator<'i, T: 'i> {
    next: Option<&'i Node<T>>
}

impl<'i, T> Iterator for StackIterator<'i, T> {
    type Item = &'i T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.val
        })
    }
}

impl <'mi, T> IntoIterator for &'mi mut Stack<T> {
    type Item = &'mi mut T;
    type IntoIter = StackMutIterator<'mi, T>;

    fn into_iter(self) -> Self::IntoIter {
        StackMutIterator { next: self.head.as_mut().map(|node| { &mut **node }) }
    }
}

pub struct StackMutIterator<'mi, T: 'mi> {
    next: Option<&'mi mut Node<T>>
}

impl<'mi, T> Iterator for StackMutIterator<'mi, T> {
    type Item = &'mi mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_mut().map(|node| &mut **node);
            &mut node.val
        })
    }
}

impl <T> IntoIterator for Stack<T> {
    type Item = T;
    type IntoIter = StackIntoIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        StackIntoIterator { stack: self }
    }
}

pub struct StackIntoIterator<T> {
    stack: Stack<T>
}

impl<T> Iterator for StackIntoIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_an_empty_stack() {
        let mut stack: Stack<i32> = Stack::new();

        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn adds_an_element_to_a_stack() {
        let mut stack = Stack::new();

        stack.push(1);

        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn adds_elements_to_a_stack() {
        let mut stack = Stack::new();

        stack.push(10);
        stack.push(20);
        stack.push(30);

        assert_eq!(stack.pop(), Some(30));
        assert_eq!(stack.pop(), Some(20));
        assert_eq!(stack.pop(), Some(10));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn peeks_head_value_from_a_stack() {
        let mut stack = Stack::new();

        stack.push(10);
        stack.push(20);

        assert_eq!(stack.peek(), Some(&20));
        assert_eq!(stack.peek(), Some(&20));
    }

    #[test]
    fn peeks_mutable_head_value_from_a_stack() {
        let mut stack = Stack::new();

        stack.push(10);
        stack.push(20);

        assert_eq!(stack.peek_mut(), Some(&mut 20));
        assert_eq!(stack.peek_mut(), Some(&mut 20));
    }

    #[test]
    fn creates_iterator_from_a_stack() {
        let mut stack = Stack::new();

        stack.push(10);
        stack.push(20);
        stack.push(30);

        let ref_stack = &stack;

        let mut iterator = ref_stack.into_iter();

        assert_eq!(iterator.next(), Some(&30));
        assert_eq!(iterator.next(), Some(&20));
        assert_eq!(iterator.next(), Some(&10));
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn creates_mutable_iterator_from_a_stack() {
        let mut stack = Stack::new();

        stack.push(10);
        stack.push(20);
        stack.push(30);

        let mut_ref_stack = &mut stack;

        let mut iterator = mut_ref_stack.into_iter();

        assert_eq!(iterator.next(), Some(&mut 30));
        assert_eq!(iterator.next(), Some(&mut 20));
        assert_eq!(iterator.next(), Some(&mut 10));
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn creates_iterator_owns_a_stack() {
        let mut stack = Stack::new();

        stack.push(10);
        stack.push(20);
        stack.push(30);

        let mut iterator = stack.into_iter();

        assert_eq!(iterator.next(), Some(30));
        assert_eq!(iterator.next(), Some(20));
        assert_eq!(iterator.next(), Some(10));
        assert_eq!(iterator.next(), None);
    }
}