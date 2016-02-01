pub use tdd_kata::queue_kata::day_5::Queue;

describe! queue_tests {

    before_each {
        let mut queue = Queue::new();
    }

    it "should create a new empty queue" {
        assert!(queue.is_empty());
        assert_eq!(queue.len(), 0);
    }

    it "should increment size when enqueue value" {
        queue.enqueue(1);

        assert!(!queue.is_empty());
    }

    it "should contain enqueued value" {
        queue.enqueue(10);

        assert!(queue.contains(10));
    }

    it "should not contain not enqueued value" {
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

    it "should dequeue None from empty queue" {
        assert_eq!(queue.dequeue(), None);
    }

    it "should decrease size when dequeue from queue" {
        queue.enqueue(1);

        queue.dequeue();

        assert!(queue.is_empty());
    }

    it "should dequeue enqueued values" {
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
