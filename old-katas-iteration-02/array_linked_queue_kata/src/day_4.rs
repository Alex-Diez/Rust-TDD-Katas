use std::rc::Rc;
use std::cell::RefCell;

const SEGMENT_SIZE: usize = 16;

#[derive(Debug, PartialEq)]
struct Segment {
    head: usize,
    tail: usize,
    items: [i32; SEGMENT_SIZE],
    next: Option<Rc<RefCell<Segment>>>
}

impl Segment {
    fn remove(&mut self) -> Option<i32> {
        if self.is_exhausted() {
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

    fn new() -> Rc<RefCell<Segment>> {
        Rc::new(RefCell::new(Segment { head: 0, tail: 0, items: [0; SEGMENT_SIZE], next: None }))
    }

    fn is_exhausted(&self) -> bool {
        self.head == self.tail
    }

    fn is_full(&self) -> bool {
        self.tail == SEGMENT_SIZE
    }
}

pub struct ArrayLinkedQueue {
    first: Option<Rc<RefCell<Segment>>>,
    last: Option<Rc<RefCell<Segment>>>
}

impl ArrayLinkedQueue {
    pub fn dequeue(&mut self) -> Option<i32> {
        self.first.take().and_then(|first| {
            if first.borrow().is_exhausted() {
                match first.borrow_mut().next.take() {
                    Some(next) => {
                        self.first = Some(next.clone());
                        next.borrow_mut().remove()
                    },
                    None => {
                        self.last.take();
                        None
                    }
                }
            } else {
                self.first = Some(first.clone());
                first.borrow_mut().remove()
            }
        })
    }

    pub fn enqueue(&mut self, item: i32) {
        let segment = match self.last.take() {
            Some(last) => {
                if last.borrow().is_full() {
                    let mut segment = Segment::new();
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
                let mut segment = Segment::new();
                segment.borrow_mut().add(item);
                self.first = Some(segment.clone());
                segment
            }
        };
        self.last = Some(segment);
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
