use std::rc::Rc;
use std::cell::RefCell;

const SEGMENT_SIZE: usize = 16;

type SegmentLink<T> = Rc<RefCell<Segment<T>>>;

#[derive(Debug, PartialEq)]
struct Segment<T: PartialEq> {
    head: usize,
    tail: usize,
    items: [T; SEGMENT_SIZE],
    next: Option<SegmentLink<T>>
}

impl<T: Default + Copy + PartialEq> Segment<T> {
    fn remove(&mut self) -> Option<T> {
        if self.head == self.tail {
            None
        } else {
            let head = self.head;
            self.head += 1;
            Some(self.items[head])
        }
    }

    fn add(&mut self, item: T) {
        let tail = self.tail;
        self.tail += 1;
        self.items[tail] = item;
    }

    fn new() -> Rc<RefCell<Segment<T>>> {
        Rc::new(
            RefCell::new(
                Segment {
                    head: 0,
                    tail: 0,
                    items: [Default::default(); SEGMENT_SIZE],
                    next: None
                }
            )
        )
    }
}

pub struct ArrayLinkedQueue<T: PartialEq> {
    first: Option<SegmentLink<T>>,
    last: Option<SegmentLink<T>>
}

impl<T: Default + Copy + PartialEq> ArrayLinkedQueue<T> {
    pub fn dequeue(&mut self) -> Option<T> {
        self.first.take().and_then(|first| {
            let head = first.borrow_mut().remove();
            let (item, segment) = head.map_or_else(
                || {
                    first.borrow_mut()
                            .next.take()
                            .map_or_else(
                                || (None, self.last.take()),
                                |next| (next.borrow_mut().remove(), Some(next.clone()))
                            )
                },
                |item| (Some(item), Some(first.clone()))
            );
            self.first = segment;
            item
        })
    }

    pub fn enqueue(&mut self, item: T) {
        let segment = match self.last.take() {
            Some(last) => {
                let tail = last.borrow().tail;
                if tail == SEGMENT_SIZE {
                    let segment = Segment::new();
                    segment.borrow_mut().add(item);
                    last.borrow_mut().next = Some(segment.clone());
                    segment
                } else {
                    last.borrow_mut().add(item);
                    if self.first.as_ref().map_or(true, |first| first == &last) {
                        self.first = Some(last.clone());
                    }
                    last
                }
            }
            None => {
                let segment = Segment::new();
                segment.borrow_mut().add(item);
                self.first = Some(segment.clone());
                segment
            }
        };
        self.last = Some(segment);
    }
}

impl<T: PartialEq> Default for ArrayLinkedQueue<T> {
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
    fn enqueue_dequeue_items_more_than_segment() {
        let mut queue = ArrayLinkedQueue::default();

        for i in 0..(2 * SEGMENT_SIZE + 1) {
            queue.enqueue(i);
        }

        for i in 0..(2 * SEGMENT_SIZE + 1) {
            assert_eq!(queue.dequeue(), Some(i));
        }
        assert_eq!(queue.dequeue(), None);
    }

    #[test]
    fn enqueue_dequeue_one_by_one_more_than_segment() {
        let mut queue = ArrayLinkedQueue::default();

        for i in 0..(SEGMENT_SIZE + 1) {
            queue.enqueue(i);
            assert_eq!(queue.dequeue(), Some(i));
            assert_eq!(queue.dequeue(), None);
        }
    }
}
