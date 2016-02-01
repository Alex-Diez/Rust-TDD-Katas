pub use tdd_kata::stack_kata::day_4::Stack;

describe! stack_tests {

    before_each {
        let mut stack: Stack<i32> = Stack::new(16);
    }

    it "should create a new empty stack" {
        assert!(stack.is_empty());
        assert_eq!(stack.size(), 0);
    }

    it "should increase the stack size when push into it" {
        let old_size = stack.size();
        stack.push(1);

        assert_eq!(old_size + 1, stack.size());
    }

    it "should decrease the stach size when pop from it" {
        stack.push(1);
        let old_size = stack.size();
        stack.pop();

        assert_eq!(old_size - 1, stack.size());
    }

    it "should pop pushed value" {
        stack.push(1);

        assert_eq!(stack.pop(), Some(1));
    }

    it "should pop first pushed last time value" {
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