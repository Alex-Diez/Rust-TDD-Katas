use std::rc::Rc;
use std::cell::RefCell;

const SEGMENT_SIZE: usize = 16;

type SegmentLink = Rc<RefCell<Segment>>;

#[derive(PartialEq)]
struct Segment {
    head: usize,
    tail: usize,
    items: [i32; SEGMENT_SIZE],
    next: Option<SegmentLink>
}

impl Segment {
    fn new() -> SegmentLink {
        Rc::new(RefCell::new(Segment { head: 0, tail: 0, items: [0; 16], next: None }))
    }

    fn with(item: i32) -> SegmentLink {
        let segment = Segment::new();
        segment.borrow_mut().tail += 1;
        segment.borrow_mut().items[0] = item;
        segment
    }

    fn add(&mut self, item: i32) -> Result<(), SegmentLink> {
        if self.tail == SEGMENT_SIZE {
            let segment = Segment::with(item);
            self.next = Some(segment.clone());
            Err(segment)
        } else {
            let tail = self.tail;
            self.tail += 1;
            self.items[tail] = item;
            Ok(())
        }
    }

    fn remove(&mut self) -> Result<i32, Option<SegmentLink>> {
        if self.tail == self.head {
            Err(self.next.take())
        } else {
            let head = self.head;
            self.head += 1;
            Ok(self.items[head])
        }
    }
}

pub struct ArrayLinkedQueue {
    first: Option<SegmentLink>,
    last: Option<SegmentLink>
}

impl ArrayLinkedQueue {
    pub fn dequeue(&mut self) -> Option<i32> {
        self.first.take().and_then(|first| {
            match first.borrow_mut().remove() {
                Ok(item) => {
                    self.first = Some(first.clone());
                    Some(item)
                }
                Err(Some(next)) => {
                    self.first = Some(next.clone());
                    next.borrow_mut().remove().ok()
                }
                Err(_) => None
            }
        })
    }

    pub fn enqueue(&mut self, item: i32) {
        match self.insert(item) {
            Ok(()) => (),
            Err(last) => self.last = Some(last.clone())
        }
    }

    fn insert(&mut self, item: i32) -> Result<(), SegmentLink> {
        let segment = self.last.get_or_insert(Segment::new());
        if self.first.as_ref().map_or(true, |first| first == segment) {
            self.first = Some(segment.clone());
        }
        segment.borrow_mut().add(item)
    }
}

impl Default for ArrayLinkedQueue {
    fn default() -> Self {
        ArrayLinkedQueue { first: None, last: None }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enqueue_dequeue_many_items() {
        let mut queue = ArrayLinkedQueue::default();

        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);

        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), Some(3));
        assert_eq!(queue.dequeue(), None);
    }

    #[test]
    fn enqueue_dequeue_more_than_segment() {
        let mut queue = ArrayLinkedQueue::default();

        for i in 0..(2 * SEGMENT_SIZE + 1) {
            queue.enqueue(i as i32);
        }

        for i in 0..(2 * SEGMENT_SIZE + 1) {
            assert_eq!(queue.dequeue(), Some(i as i32));
        }
        assert_eq!(queue.dequeue(), None);
    }

    #[test]
    fn enqueue_dequeue_one_by_one_more_than_segment() {
        let mut queue = ArrayLinkedQueue::default();

        for i in 0..(2 * SEGMENT_SIZE + 1) {
            queue.enqueue(i as i32);
            assert_eq!(queue.dequeue(), Some(i as i32));
            assert_eq!(queue.dequeue(), None);
        }
    }
}
