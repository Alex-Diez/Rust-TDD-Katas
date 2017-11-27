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
    fn new() -> SegmentLink {
        Rc::new(
            RefCell::new(
                Segment {
                    head: 0,
                    tail: 0,
                    items: [0; 16],
                    next: None
                }
            )
        )
    }

    fn with(item: i32) -> SegmentLink {
        let segment = Segment::new();
        let tail = segment.borrow().tail;
        segment.borrow_mut().tail += 1;
        segment.borrow_mut().items[tail] = item;
        segment
    }

    fn remove_first(&mut self) -> Result<i32, Option<SegmentLink>> {
        if self.head == self.tail {
            Err(self.next.take())
        } else {
            let head = self.head;
            self.head += 1;
            Ok(self.items[head])
        }
    }

    fn add_last(&mut self, item: i32) -> Result<(), SegmentLink> {
        if self.tail == 16 {
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
}

pub struct ArrayLinkedQueue {
    first: Option<SegmentLink>,
    last: Option<SegmentLink>
}

impl ArrayLinkedQueue {
    pub fn dequeue(&mut self) -> Option<i32> {
        self.first.take().and_then(|first| {
            match first.borrow_mut().remove_first() {
                Ok(item) => {
                    self.first = Some(first.clone());
                    Some(item)
                },
                Err(Some(next)) => {
                    self.first = Some(next.clone());
                    next.borrow_mut().remove_first().ok()
                }
                Err(_) => None
            }
        })
    }

    pub fn enqueue(&mut self, item: i32) {
        self.insert(item).map_err(|segment| self.last = Some(segment));
    }

    fn insert(&mut self, item: i32) -> Result<(), SegmentLink> {
        let last = self.last.get_or_insert(Segment::new());
        if self.first.as_ref().map_or(true, |first| first == last) {
            self.first = Some(last.clone());
        }
        last.borrow_mut().add_last(item)
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
    fn enqueue_dequeue_many() {
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

        for i in 0..(16 + 1) {
            queue.enqueue(i as i32);
            assert_eq!(queue.dequeue(), Some(i as i32));
            assert_eq!(queue.dequeue(), None);
        }
    }
}
