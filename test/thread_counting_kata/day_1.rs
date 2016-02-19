pub use tdd_kata::thread_counting_kata::day_1::{Counter, ThreadCounter};

pub use expectest::prelude::be_equal_to;

describe! thread_counting_tests {

    it "should create a thread counter" {
        ThreadCounter::new(3, 100);
    }

    ignore "should count from 1 till 100" {
        let thread_counter = ThreadCounter::new(3, 100);
        let mut counter = Counter::new();

        thread_counter.count_in_threads(&mut counter);

        let vec = (1..101).collect::<Vec<i32>>();

        expect!(counter.numbers()).to(be_equal_to(vec));
    }

    ignore "should count from 1 till 200" {
        let thread_counter = ThreadCounter::new(3, 200);
        let mut counter = Counter::new();

        thread_counter.count_in_threads(&mut counter);

        let vec = (1..201).collect::<Vec<i32>>();

        expect!(counter.numbers()).to(be_equal_to(vec));
    }

    ignore "should be counted in 3 threads" {
        let thread_counter = ThreadCounter::new(3, 100);
        let mut counter = Counter::new();

        thread_counter.count_in_threads(&mut counter);

        expect!(counter.threads()).to(be_equal_to(3));
    }

    ignore "should be counted in 5 threads" {
        let thread_counter = ThreadCounter::new(5, 100);
        let mut counter = Counter::new();

        thread_counter.count_in_threads(&mut counter);

        expect!(counter.threads()).to(be_equal_to(5));
    }
}
