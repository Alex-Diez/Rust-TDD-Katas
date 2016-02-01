#![feature(plugin,const_fn)]
#![plugin(stainless)]

extern crate collections;

pub use collections::Stack;

describe! stack_tests {

    before_each {
        let mut stack = Stack::new(16);
    }

    it "new created stack should be empty" {
        assert!(stack.is_empty());
        assert_eq!(stack.size(), 0);
    }

    it "push into the stack modify size" {
        stack.push(1);

        assert!(!stack.is_empty());
        assert_eq!(stack.size(), 1);
    }

    it "pop from empty set should return None" {
        assert_eq!(stack.pop(), None);
    }

    it "pushed value should be pop" {
        stack.push(10);

        assert_eq!(stack.pop(), Some(10));

        stack.push(20);

        assert_eq!(stack.pop(), Some(20));
    }

    it "pushed first should be pop last" {
        stack.push(10);
        stack.push(20);

        assert_eq!(stack.pop(), Some(20));
        assert_eq!(stack.pop(), Some(10));
    }

    describe! fill_stack_tests {

        before_each {
            for i in 0..10 {
                stack.push(i);
            };
        }

        it "pop from the stack modify size" {
            stack.pop();

            assert_eq!(stack.size(), 9);
        }
    }
}