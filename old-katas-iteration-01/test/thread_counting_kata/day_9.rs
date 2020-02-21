pub use std::collections::HashSet;

pub use tdd_kata::thread_counting_kata::day_9::{ThreadCounter, Counter};

pub use expectest::prelude::be_equal_to;

describe! thread_counting_tests {

    before_each {
        let counter = Counter::new();
    }

    it "should count from 1 till 100" {
        let thread_counter = ThreadCounter::new(3, 100);

        thread_counter.count_in_threads(&counter);

        let data = (1..).take(100).collect::<HashSet<i32>>();

        expect!(counter.numbers()).to(be_equal_to(data));
    }

    it "should count from 1 till 200" {
        let thread_counter = ThreadCounter::new(3, 200);

        thread_counter.count_in_threads(&counter);

        let data = (1..).take(200).collect::<HashSet<i32>>();

        expect!(counter.numbers()).to(be_equal_to(data));
    }

    it "should be counted in 3 threads" {
        let thread_counter = ThreadCounter::new(3, 100);

        thread_counter.count_in_threads(&counter);

        expect!(counter.threads()).to(be_equal_to(3));
    }

    it "should be counted in 5 threads" {
        let thread_counter = ThreadCounter::new(5, 100);

        thread_counter.count_in_threads(&counter);

        expect!(counter.threads()).to(be_equal_to(5));
    }

    it "should count its number set" {
        let thread_counter = ThreadCounter::new(5, 100);

        thread_counter.count_in_threads(&counter);

        let thread_data = (1..).take(100).filter(|i| i % 5 == 1).collect::<HashSet<i32>>();

        expect!(counter.numbers_of("Thread-1")).to(be_equal_to(thread_data));

        let thread_data = (1..).take(100).filter(|i| i % 5 == 2).collect::<HashSet<i32>>();

        expect!(counter.numbers_of("Thread-2")).to(be_equal_to(thread_data));

        let thread_data = (1..).take(100).filter(|i| i % 5 == 3).collect::<HashSet<i32>>();

        expect!(counter.numbers_of("Thread-3")).to(be_equal_to(thread_data));

        let thread_data = (1..).take(100).filter(|i| i % 5 == 4).collect::<HashSet<i32>>();

        expect!(counter.numbers_of("Thread-4")).to(be_equal_to(thread_data));

        let thread_data = (1..).take(100).filter(|i| i % 5 == 0).collect::<HashSet<i32>>();

        expect!(counter.numbers_of("Thread-5")).to(be_equal_to(thread_data));
    }
}
