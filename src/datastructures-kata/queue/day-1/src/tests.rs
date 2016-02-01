#![feature(plugin,const_fn)]
#![plugin(stainless)]

extern crate collections;

pub use collections::Queue;

describe! queue_tests {

    before_each {
        let mut queue = Queue::new();
    }

    it "should create new empty queue" {
        assert!(queue.is_empty());
        assert_eq!(queue.size(), 0);
    }

    it "should increase size when enqueue" {
        queue.enqueue(1);

        assert!(!queue.is_empty());
    }

    it "should decrease size when dequeue" {
        queue.enqueue(1);

        queue.dequeue();
        assert!(queue.is_empty());
    }

    it "should contains value that was enqueued" {
        queue.enqueue(1);
        assert!(queue.contains(1));
    }

    it "should not contains value that was not enqueued" {
        queue.enqueue(2);
        assert!(!queue.contains(1));
    }

    it "should contains values that were enqueued" {
        queue.enqueue(10);
        queue.enqueue(20);
        queue.enqueue(30);

        assert!(queue.contains(10));
        assert!(queue.contains(20));
        assert!(queue.contains(30));
    }
}