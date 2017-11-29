use std::rc::Rc;
use std::cell::RefCell;

const SEGMENT_SIZE: usize = 16;

type SegmentLink<T> = Rc<RefCell<Segment<T>>>;

#[derive(PartialEq)]
struct Segment<T> {
    head: usize,
    tail: usize,
    items: [T; SEGMENT_SIZE],
    next: Option<SegmentLink<T>>
}

impl<T: Default + Copy> Segment<T> {
    fn new() -> SegmentLink<T> {
        Rc::new(
            RefCell::new(
                Segment { head: 0, tail: 0, items: [T::default(); SEGMENT_SIZE], next: None }
            )
        )
    }

    fn with(item: T) -> SegmentLink<T> {
        let segment = Segment::new();
        segment.borrow_mut().tail += 1;
        segment.borrow_mut().items[0] = item;
        segment
    }

    fn add(&mut self, item: T) -> Result<(), SegmentLink<T>> {
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

    fn remove(&mut self) -> Result<T, Option<SegmentLink<T>>> {
        if self.head == self.tail {
            Err(self.next.take())
        } else {
            let head = self.head;
            self.head += 1;
            Ok(self.items[head])
        }
    }
}

pub struct ArrayLinkedQueue<T> {
    first: Option<Rc<RefCell<Segment<T>>>>,
    last: Option<Rc<RefCell<Segment<T>>>>
}

impl<T: Default + Copy + PartialEq> ArrayLinkedQueue<T> {
    pub fn dequeue(&mut self) -> Option<T> {
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

    pub fn enqueue(&mut self, item: T) {
        match self.insert(item) {
            Ok(()) => (),
            Err(next) => self.last = Some(next.clone())
        }
    }

    fn insert(&mut self, item: T) -> Result<(), SegmentLink<T>> {
        let segment = self.last.get_or_insert(Segment::new());
        if self.first.as_ref().map_or(true, |first| first == segment) {
            self.first = Some(segment.clone());
        }
        segment.borrow_mut().add(item)
    }
}

impl<T: Default> Default for ArrayLinkedQueue<T> {
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
        }
        assert_eq!(queue.dequeue(), None);
    }
}
