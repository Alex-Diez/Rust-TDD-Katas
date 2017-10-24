pub use tdd_kata::stack_kata::day_1::Stack;

describe! stack_tests {

    before_each {
        let mut stack: Stack<i32> = Stack::new(10);
    }

    it "created stack should be empty" {
        assert_eq!(stack.size(), 0);
        assert_eq!(stack.is_empty(), true);
    }

    it "a pushed element should be pop" {
        stack.push(1);

        assert_eq!(stack.pop(), Some(1));
    }

    it "push to stack make its size bigger" {
        stack.push(1);

        assert_eq!(stack.is_empty(), false);
        assert_eq!(stack.size(), 1);
    }

    it "pushed first should be pop last" {
        stack.push(10);
        stack.push(20);

        assert_eq!(stack.pop(), Some(20));
        assert_eq!(stack.pop(), Some(10));
    }

    describe! filled_stack_tests {

        it "pop from stack make its size smaller" {
            for i in 0..10 {
                stack.push(i);
            }
            let before_size = stack.size();
            println!("old size - {:?}", before_size);
            stack.pop();
            println!("new size - {:?}", stack.size());

            assert_eq!(stack.size(), before_size - 1);
        }

        failing "push in filled stack should panic" {
            for i in 0..10 {
                stack.push(i);
            }
            stack.push(20)
        }
    }

}
