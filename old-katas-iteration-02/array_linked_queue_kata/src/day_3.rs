use std::rc::Rc;
use std::cell::RefCell;

type SegmentLink = Rc<RefCell<Segment>>;

#[derive(Debug, PartialEq)]
struct Segment {
    head: usize,
    tail: usize,
    items: [i32; 16],
    next: Option<SegmentLink>
}

impl Segment {
    fn new(item: i32) -> SegmentLink {
        let mut segment = Segment { head: 0, tail: 1, items: [0; 16], next: None };
        segment.items[0] = item;
        Rc::new(RefCell::new(segment))
    }

    fn add(&mut self, item: i32) {
        let tail = self.tail;
        self.tail += 1;
        self.items[tail] = item;
    }

    fn is_empty(&self) -> bool {
        self.head == self.tail
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

    fn is_full(&self) -> bool {
        self.tail == 16
    }
}

pub struct ArrayLinkedQueue {
    first: Option<Rc<RefCell<Segment>>>,
    last: Option<Rc<RefCell<Segment>>>
}

impl ArrayLinkedQueue {
    pub fn dequeue(&mut self) -> Option<i32> {
        self.first.take().and_then(|first| {
            if first.borrow().is_empty() {
                match first.borrow_mut().next.take() {
                    Some(next) => {
                        self.first = Some(next.clone());
                        self.last = Some(next.clone());
                        next.borrow_mut().remove()
                    },
                    None => {
                        self.last.take();
                        None
                    }
                }
            } else {
                self.first = Some(first.clone());
                self.last = Some(first.clone());
                first.borrow_mut().remove()
            }
        })
    }

    pub fn enqueue(&mut self, item: i32) {
        let segment = match self.last.take() {
            Some(last) => {
                if last.borrow().is_full() {
                    let segment = Segment::new(item);
                    last.borrow_mut().next = Some(segment.clone());
                    if self.first.as_ref().map_or(true, |first| first == &last) {
                        self.first = Some(last.clone());
                    }
                    segment
                } else {
                    last.borrow_mut().add(item);
                    if self.first.as_ref().map_or(true, |first| first == &last) {
                        self.first = Some(last.clone());
                    }
                    last
                }
            },
            None => {
                let segment = Segment::new(item);
                self.first = Some(segment.clone());
                segment
            }
        };
        self.last = Some(segment.clone());
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
    fn enqueue_dequeue_more_than_segment() {
        let mut queue = ArrayLinkedQueue::default();

        for i in 0..(2 * 16 + 1) {
            queue.enqueue(i as i32);
        }

        for i in 0..(2 * 16 + 1) {
            assert_eq!(queue.dequeue(), Some(i as i32));
        }
        assert_eq!(queue.dequeue(), None);
    }

    #[test]
    fn enqueue_dequeue_one_by_one_more_than_segment() {
        let mut queue = ArrayLinkedQueue::default();

        for i in 0..(2 * 16 + 1) {
            queue.enqueue(i as i32);
            assert_eq!(queue.dequeue(), Some(i as i32));
            assert_eq!(queue.dequeue(), None);
        }
        assert_eq!(queue.dequeue(), None);
    }
}
