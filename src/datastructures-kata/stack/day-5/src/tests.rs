#![feature(plugin, const_fn)]
#![plugin(stainless)]

extern crate collections;

pub use collections::Stack;

describe! stack_test {

    before_each {
        let mut stack: Stack<i32> = Stack::new(16);
    }

    it "should create a new empty stack" {
        assert_eq!(stack.size(), 0);
        assert!(stack.is_empty());
    }

    it "should increase size of the stack when push into it" {
        let old_size = stack.size();
        stack.push(1);

        assert!(!stack.is_empty());
        assert_eq!(old_size, stack.size() - 1);
    }

    it "should decrease size of the stack when pop from it" {
        stack.push(1);
        let old_size = stack.size();
        stack.pop();

        assert!(stack.is_empty());
        assert_eq!(old_size, stack.size() + 1);
    }

    it "should pop from pushed into the stack value" {
        stack.push(10);
        assert_eq!(stack.pop(), Some(10));

        stack.push(20);
        assert_eq!(stack.pop(), Some(20));
    }

    it "should pop last pushed first into the stack value" {
        stack.push(10);
        stack.push(20);
        stack.push(30);
        stack.push(40);

        assert_eq!(stack.pop(), Some(40));
        assert_eq!(stack.pop(), Some(30));
        assert_eq!(stack.pop(), Some(20));
        assert_eq!(stack.pop(), Some(10));
    }
}