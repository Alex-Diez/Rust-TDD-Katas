pub use std::collections::HashSet;

pub use tdd_kata::thread_counting_kata::day_3::{ThreadCounter, Counter};

pub use expectest::prelude::be_equal_to;

describe! thread_counter_tests {

    before_each {
        let counter = Counter::new();
    }

    it "should count from 1 till 100" {
        let thread_counter = ThreadCounter::new(3, 100);

        thread_counter.count_in_threads(&counter);

        let data = (1..101).collect::<HashSet<i32>>();

        expect!(counter.numbers()).to(be_equal_to(data));
    }

    it "should count from 1 till 200" {
        let thread_counter = ThreadCounter::new(3, 200);

        thread_counter.count_in_threads(&counter);

        let data = (1..201).collect::<HashSet<i32>>();

        expect!(counter.numbers()).to(be_equal_to(data));
    }

    it "should be couting in 3 threads" {
        let thread_counter = ThreadCounter::new(3, 100);

        thread_counter.count_in_threads(&counter);

        expect!(counter.threads()).to(be_equal_to(3));
    }

    it "should be counting in 5 threads" {
        let thread_counter = ThreadCounter::new(5, 100);

        thread_counter.count_in_threads(&counter);

        expect!(counter.threads()).to(be_equal_to(5));
    }

    it "should count specified set of numbers" {
        let thread_counter = ThreadCounter::new(3, 100);

        thread_counter.count_in_threads(&counter);

        let numbers = 1..;
        let thread_data = numbers.take(100).filter(|i| i % 3 == 1).collect::<HashSet<i32>>();
        let thread_name = format!("Thread-{}", 0);

        expect!(counter.thread_numbers(thread_name)).to(be_equal_to(thread_data));

        let numbers = 1..;
        let thread_data = numbers.take(100).filter(|i| i % 3 == 2).collect::<HashSet<i32>>();
        let thread_name = format!("Thread-{}", 1);

        let numbers = 1..;
        let thread_data = numbers.take(100).filter(|i| i % 3 == 0).collect::<HashSet<i32>>();
        let thread_name = format!("Thread-{}", 2);

        expect!(counter.thread_numbers(thread_name)).to(be_equal_to(thread_data));
    }
}
