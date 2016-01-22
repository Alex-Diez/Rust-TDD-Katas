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
        assert_eq!(queue.len(), 0);
    }

    it "should increase size when enqueue" {
        queue.enqueue(1);
        assert!(!queue.is_empty());
    }

    it "should contains enqueued elem" {
        queue.enqueue(1);
        assert!(queue.contains(1));
    }

    it "should not contains not enqueued elem" {
        assert!(!queue.contains(1))
    }

    it "should contains enqueued elements" {
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

    it "should dequeue none from empty queue" {
        assert_eq!(queue.dequeue(), None);
    }

    it "should dequeue enqueued elem" {
        queue.enqueue(10);

        assert_eq!(queue.dequeue(), Some(10));
    }

    it "should dequeue in order enqueue" {
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