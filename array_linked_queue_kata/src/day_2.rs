use std::rc::Rc;
use std::cell::RefCell;

const SEGMENT_SIZE: usize = 16;

type SegmentLink = Rc<RefCell<Segment>>;

#[derive(Debug, PartialEq)]
struct Segment {
    head: usize,
    tail: usize,
    items: [i32; SEGMENT_SIZE],
    next: Option<SegmentLink>
}

impl Segment {
    fn new(item: i32) -> SegmentLink {
        let mut segment = Segment { head: 0, tail: 0, items: [0; SEGMENT_SIZE], next: None };
        segment.tail += 1;
        segment.items[0] = item;
        Rc::new(RefCell::new(segment))
    }

    fn is_empty(&self) -> bool {
        self.head == self.tail
    }

    fn is_full(&self) -> bool {
        self.tail == SEGMENT_SIZE
    }

    fn is_exhausted(&self) -> bool {
        self.head == SEGMENT_SIZE
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

    fn add(&mut self, item: i32) {
        let tail = self.tail;
        self.tail += 1;
        self.items[tail] = item;
    }
}

pub struct ArrayLinkedQueue {
    first: Option<SegmentLink>,
    last: Option<SegmentLink>
}

impl ArrayLinkedQueue {
    pub fn dequeue(&mut self) -> Option<i32> {
        self.first.take().and_then(|first| {
            let exhausted = first.borrow().is_exhausted();
            let segment = if exhausted {
                first.borrow_mut().next.take()
            } else {
                Some(first)
            };
            self.first = segment.clone();
            self.last = segment.clone();
            segment.and_then(|segment| segment.borrow_mut().remove())
        })
    }

    pub fn enqueue(&mut self, item: i32) {
        let segment = match self.last.take() {
            Some(last) => {
                if last.borrow().is_full() {
                    let segment = Segment::new(item);
                    last.borrow_mut().next = Some(segment.clone());
                    if self.first.as_ref().map_or(true, |first| first == &last) {
                        self.first = Some(last);
                    }
                    segment
                } else {
                    last.borrow_mut().add(item);
                    last
                }
            },
            None => {
                let segment = Segment::new(item);
                self.first = Some(segment.clone());
                segment
            }
        };
        self.last = Some(segment);
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
    fn enqueue_deque_more_than_segment_capacity() {
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
    fn enqueue_double_dequeue_one_by_one_more_than_segment() {
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
