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
    fn with(item: i32) -> SegmentLink {
        let mut segment = Segment {
            head: 0,
            tail: 0,
            items: [0; SEGMENT_SIZE],
            next: None
        };
        segment.tail += 1;
        segment.items[0] = item;
        Rc::new(
            RefCell::new(
                segment
            )
        )
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
        if self.head == self.tail {
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
            let item = first.borrow_mut().remove();
            match item {
                Ok(item) => {
                    self.first = Some(first.clone());
                    Some(item)
                },
                Err(next) => {
                    match next {
                        Some(next) => {
                            self.first = Some(next.clone());
                            next.borrow_mut().remove().ok()
                        },
                        None => {
                            self.last.take();
                            None
                        }
                    }
                }
            }
        })
    }

    pub fn enqueue(&mut self, item: i32) {
        let segment = match self.last.take() {
            Some(last) => {
                let ret = last.borrow_mut().add(item);
                match ret {
                    Ok(()) if self.single_segment(&last) => {
                        self.first = Some(last.clone());
                        last
                    },
                    Ok(()) => last,
                    Err(segment) => segment
                }
            }
            None => {
                let mut segment = Segment::with(item);
                self.first = Some(segment.clone());
                segment
            }
        };
        self.last = Some(segment.clone());
    }

    fn single_segment(&self, ref_last: &SegmentLink) -> bool {
        self.first.as_ref().map_or(true, |first| first == ref_last)
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
