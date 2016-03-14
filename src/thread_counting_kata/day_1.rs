#![allow(unused_variables, dead_code, unused_imports, unused_variables, new_without_default)]

use std::collections::HashSet;
use std::sync::{Barrier, Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};

use std::thread;

#[derive(Clone)]
pub struct Counter {
    numbers: Vec<i32>,
    threads_name: HashSet<Option<String>>
}

impl Counter {

    pub fn new() -> Counter {
        Counter {
            numbers: Vec::new(),
            threads_name: HashSet::new()
        }
    }

    pub fn numbers(&self) -> Vec<i32> {
        self.numbers.iter().cloned().collect::<Vec<i32>>()
    }

    pub fn threads(&self) -> usize {
        self.threads_name.len()
    }

    fn count(&mut self, n: i32) {
        self.numbers.push(n);
        let name = thread::current().name().map(|s| s.to_owned());
        self.threads_name.insert(name);
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

    pub fn count_in_threads(&self, counter: &mut Counter) {
/*        let counter = Arc::new(Mutex::new(counter));
        let mut flags = Vec::with_capacity(self.number_of_threads);
        for _ in 0..self.number_of_threads {
            flags.push(Arc::new(AtomicBool::new(false)));
        }
        flags[0].store(true, Ordering::Relaxed);
        let barrier = Arc::new(Barrier::new(self.number_of_threads+1));
        for i in 0..self.number_of_threads {
            let barrier = barrier.clone();
            let start = i + 1;
            let limit = self.limit.clone() as i32;
            let allow_flag = flags[i % self.number_of_threads].clone();
            let readiness_flag = flags[(i+1) % self.number_of_threads].clone();
            let counter = counter.clone();
            let increment = self.number_of_threads.clone() as i32;
            thread::Builder::new().name(format!("Thread-{}", i)).spawn(
                move || {
                    let mut current = start as i32;
                    while current <= limit {
                        while !allow_flag.load(Ordering::Acquire) {
                        }
                        let mut lock = counter.lock().unwrap();
                        lock.count(current);
                        drop(lock);
                        readiness_flag.store(true, Ordering::SeqCst);
                        allow_flag.store(false, Ordering::Release);
                        current += increment;
                    }
                    barrier.wait();
                }
            );
        }
        barrier.wait();*/
    }
}
