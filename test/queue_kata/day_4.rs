pub use tdd_kata::queue_kata::day_4::Queue;

describe! queue_tests {

    before_each {
        let mut queue = Queue::new();
    }

    it "should create new empty queue" {
        assert!(queue.is_empty());
        assert_eq!(queue.len(), 0);
    }

    it "should increase size when enqueue value" {
        queue.enqueue(1);

        assert!(!queue.is_empty());
    }

    it "should contain value that was enqueued" {
        queue.enqueue(1);

        assert!(queue.contains(1));
    }

    it "should not contain value that was not enqueued" {
        assert!(!queue.contains(2));
    }

    it "should contain values that were enqueued" {
        queue.enqueue(10);
        queue.enqueue(20);
        queue.enqueue(30);
        queue.enqueue(40);

        assert!(queue.contains(10));
        assert!(queue.contains(20));
        assert!(queue.contains(30));
        assert!(queue.contains(40));
    }

    it "should decrease size when dequeue value" {
        queue.enqueue(10);

        queue.dequeue();
        assert!(queue.is_empty());
    }

    it "should dequeue None value from empty queue" {
        assert_eq!(queue.dequeue(), None);
    }

    it "should dequeue enqueued value" {
        queue.enqueue(10);

        assert_eq!(queue.dequeue(), Some(10));
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