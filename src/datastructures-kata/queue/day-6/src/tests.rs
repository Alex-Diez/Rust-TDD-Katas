#![feature(const_fn,plugin)]
#![plugin(stainless)]

extern crate collections;

pub use self::collections::Queue;

describe! queue_test {

    before_each {
        let mut queue = Queue::new();
    }

    it "should create a queue" {
        assert_eq!(queue.len(), 0);
        assert!(queue.is_empty());
    }

    it "should increase size when enqueue" {
        queue.enqueue(1);

        assert!(!queue.is_empty());
    }

    it "should contain value that was enqueued" {
        queue.enqueue(1);

        assert!(queue.contains(1));
    }

    it "should not contain value that was not enqueue" {
        assert!(!queue.contains(20));
    }

    it "should contain all enqueued values" {
        queue.enqueue(10);
        queue.enqueue(20);
        queue.enqueue(30);
        queue.enqueue(40);

        assert!(queue.contains(10));
        assert!(queue.contains(20));
        assert!(queue.contains(30));
        assert!(queue.contains(40));
    }

    it "should decrease size when dequeue" {
        queue.enqueue(1);

        queue.dequeue();

        assert!(queue.is_empty());
    }

    it "should dequeue None from empty queue" {
        assert_eq!(queue.dequeue(), None);
    }

    it "should dequeue enqueued value" {
        queue.enqueue(10);

        assert_eq!(queue.dequeue(), Some(10));
    }

    it "should dequeue all enqueued values" {
        queue.enqueue(10);
        queue.enqueue(20);
        queue.enqueue(30);
        queue.enqueue(40);

        assert_eq!(queue.dequeue(), Some(10));
        assert_eq!(queue.dequeue(), Some(20));
        assert_eq!(queue.dequeue(), Some(30));
        assert_eq!(queue.dequeue(), Some(40));
    }
}
