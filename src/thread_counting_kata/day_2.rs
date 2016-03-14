#![allow(new_without_default)]

use std::collections::HashSet;
use std::sync::{Mutex, Arc, Barrier};
use std::sync::atomic::{AtomicBool, Ordering};

use std::thread;

struct CounterInternal {
    numbers: Vec<i32>,
    threads_name: HashSet<Option<String>>
}

impl CounterInternal {

    fn new() -> CounterInternal {
        CounterInternal {
            numbers: Vec::new(),
            threads_name: HashSet::new()
        }
    }

    fn numbers(&self) -> HashSet<i32> {
        self.numbers.iter().cloned().collect::<HashSet<i32>>()
    }

    fn threads(&self) -> usize {
        self.threads_name.len()
    }

    fn count(&mut self, n: i32) {
        let thread_name = thread::current().name().map(|n| n.to_owned());
        self.threads_name.insert(thread_name);
        self.numbers.push(n);
    }
}

#[derive(Clone)]
pub struct Counter {
    internal: Arc<Mutex<CounterInternal>>
}

impl Counter {

    pub fn new() -> Counter {
        Counter {
            internal: Arc::new(Mutex::new(CounterInternal::new()))
        }
    }

    pub fn numbers(&self) -> HashSet<i32> {
        let guard = self.internal.lock().unwrap();
        guard.numbers()
    }

    pub fn threads(&self) -> usize {
        let guard = self.internal.lock().unwrap();
        guard.threads()
    }

    fn count(&self, n: i32) {
        let mut guard = self.internal.lock().unwrap();
        guard.count(n);
    }
}

pub struct ThreadCounter {
    number_of_threads: usize,
    limit: usize
}

impl ThreadCounter {

    pub fn new(number_of_threads: usize, limit: usize) -> ThreadCounter {
        ThreadCounter {
            number_of_threads: number_of_threads,
            limit: limit
        }
    }

    #[allow(unused_must_use)]
    pub fn count_in_threads(&self, counter: &Counter) {
        let mut flags = Vec::with_capacity(self.number_of_threads);
        for _ in 0..self.number_of_threads {
            flags.push(Arc::new(AtomicBool::new(false)));
        }
        flags[0].store(true, Ordering::SeqCst);
        let barrier = Arc::new(Barrier::new(self.number_of_threads+1));
        for i in 0..self.number_of_threads {
            let counter = counter.clone();
            let barrier = barrier.clone();
            let start = i+1;
            let limit = self.limit as i32;
            let increment = self.number_of_threads as i32;
            let allow_flag = flags[i % self.number_of_threads].clone();
            let readiness_flag = flags[(i+1) % self.number_of_threads].clone();
            let thread_name = format!("Thread-{}", i);
            thread::Builder::new().name(thread_name).spawn(
                move || {
                    let mut current = start as i32;
                    while current <= limit {
                        while !allow_flag.load(Ordering::SeqCst) {
                        }
                        counter.count(current);
                        allow_flag.store(false, Ordering::SeqCst);
                        readiness_flag.store(true, Ordering::SeqCst);
                        current += increment;
                    }
                    barrier.wait();
                }
            );
        }
        barrier.wait();
    }
}
