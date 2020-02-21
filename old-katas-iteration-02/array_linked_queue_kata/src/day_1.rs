use std::rc::Rc;
use std::cell::RefCell;

const SEGMENT_SIZE: usize = 16;

type RcSegment = Rc<RefCell<Segment>>;
type SegmentLink = Option<RcSegment>;

#[derive(Debug, PartialEq)]
struct Segment {
    head: usize,
    tail: usize,
    items: [i32; SEGMENT_SIZE],
    next: SegmentLink
}

impl Segment {
    fn new(item: i32) -> SegmentLink {
        let mut segment = Segment {
            head: 0,
            tail: 1,
            items: [0; SEGMENT_SIZE],
            next: None
        };
        segment.items[0] = item;
        Some(Rc::new(RefCell::new(segment)))
    }

    fn add(&mut self, item: i32) {
        let tail = self.tail;
        self.tail += 1;
        self.items[tail] = item;
    }

    fn remove(&mut self) -> Option<i32> {
        if self.is_empty() {
            None
        } else {
            let head = self.head;
            self.head += 1;
            Some(self.items[head])
        }
    }

    fn is_empty(&self) -> bool {
        self.head == self.tail
    }

    fn is_exhausted(&self) -> bool {
        self.head == SEGMENT_SIZE
    }

    fn is_full(&self) -> bool {
        self.tail == SEGMENT_SIZE
    }
}

pub struct ArrayLinkedQueue {
    first: SegmentLink,
    last: SegmentLink
}

impl ArrayLinkedQueue {
    pub fn dequeue(&mut self) -> Option<i32> {
        self.first.take().and_then(|segment| {
            let segment = self.get_next_segment(segment);
            self.first = segment.clone();
            segment.and_then(|segment| segment.borrow_mut().remove())
        })
    }

    fn get_next_segment(&mut self, segment: RcSegment) -> SegmentLink {
        let the_last = self.last.as_ref().map_or(true, |last| last == &segment);
        let borrowed = segment.borrow();
        if borrowed.is_empty() && the_last {
            self.last.take()
        } else if borrowed.is_exhausted() {
            borrowed.next.clone()
        } else {
            Some(segment.clone())
        }
    }

    pub fn enqueue(&mut self, item: i32) {
        match self.last.take() {
            Some(segment) => {
                let segment = if segment.borrow().is_full() {
                    let next_segment = Segment::new(item);
                    segment.borrow_mut().next = next_segment.clone();
                    self.first = Some(segment);
                    next_segment
                } else {
                    segment.borrow_mut().add(item);
                    Some(segment)
                };
                self.last = segment;
            }
            None => {
                let segment = Segment::new(item);
                self.first = segment.clone();
                self.last = segment;
            }
        }
    }
}

impl Default for ArrayLinkedQueue {
    fn default() -> Self {
        ArrayLinkedQueue {
            first: None,
            last: None
        }
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
    fn enqueue_dequeue_more_than_segment_size() {
        let mut queue = ArrayLinkedQueue::default();

        for i in 0..(SEGMENT_SIZE + 1) {
            queue.enqueue(i as i32);
        }

        for i in 0..(SEGMENT_SIZE + 1) {
            assert_eq!(queue.dequeue(), Some(i as i32));
        }
        assert_eq!(queue.dequeue(), None);
    }

    #[test]
    fn enqueue_double_dequeue_more_than_segment() {
        let mut queue = ArrayLinkedQueue::default();

        for i in 0..(2 * SEGMENT_SIZE + 1) {
            queue.enqueue(i as i32);
            assert_eq!(queue.dequeue(), Some(i as i32));
            assert_eq!(queue.dequeue(), None);
        }
    }

    #[test]
    fn enqueue_dequeue_one_by_one_more_than_segment() {
        let mut queue = ArrayLinkedQueue::default();

        for i in 0..(2 * SEGMENT_SIZE + 1) {
            queue.enqueue(i as i32);
            assert_eq!(queue.dequeue(), Some(i as i32));
        }
        assert_eq!(queue.dequeue(), None);
    }
}
