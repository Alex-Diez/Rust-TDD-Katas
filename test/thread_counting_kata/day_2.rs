pub use std::collections::HashSet;

pub use tdd_kata::thread_counting_kata::day_2::{ThreadCounter, Counter};

pub use expectest::prelude::be_equal_to;

describe! thread_counting_tests {

    it "should create a new thread counter" {
        ThreadCounter::new(3, 100);
    }

    it "should count from 1 till 100" {
        let thread_counter = ThreadCounter::new(3, 100);
        let counter = Counter::new();

        thread_counter.count_in_threads(&counter);

        let ret = (1..101).collect::<HashSet<i32>>();

        expect!(counter.numbers()).to(be_equal_to(ret));
    }

    it "should count from 1 till 200" {
        let thread_counter = ThreadCounter::new(3, 200);
        let counter = Counter::new();

        thread_counter.count_in_threads(&counter);

        let ret = (1..201).collect::<HashSet<i32>>();

        expect!(counter.numbers()).to(be_equal_to(ret));
    }

    it "should count in 3 threads" {
        let thread_counter = ThreadCounter::new(3, 100);
        let counter = Counter::new();

        thread_counter.count_in_threads(&counter);

        expect!(counter.threads()).to(be_equal_to(3));
    }

    it "should count in 5 threads" {
        let thread_counter = ThreadCounter::new(5, 100);
        let counter = Counter::new();

        thread_counter.count_in_threads(&counter);

        expect!(counter.threads()).to(be_equal_to(5));
    }
}
