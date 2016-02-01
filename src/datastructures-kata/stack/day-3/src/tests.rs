#![feature(plugin,const_fn)]
#![plugin(stainless)]

extern crate collections;

pub use collections::Stack;

describe! stack_tests {

    before_each {
        let mut stack: Stack<i32> = Stack::new(20);
    }

    it "should create an empty stack" {
        assert!(stack.is_empty());
        assert_eq!(stack.size(), 0);
    }

    it "should pop a value that was pushed" {
        stack.push(20);
        assert_eq!(stack.pop(), Some(20));

        stack.push(10);
        assert_eq!(stack.pop(), Some(10));
    }

    it "should increse size when push into the stack" {
        stack.push(20);
        assert_eq!(stack.size(), 1);
        assert_eq!(stack.is_empty(), false);
    }

    it "should decrise size when pop from the stack" {
        stack.push(10);
        let old_size = stack.size();
        stack.pop();
        assert!(stack.size() < old_size);
    }

    it "should pushed first pop last" {
        stack.push(20);
        stack.push(10);
        stack.push(30);

        assert_eq!(stack.pop(), Some(30));
        assert_eq!(stack.pop(), Some(10));
        assert_eq!(stack.pop(), Some(20));
    }
}
