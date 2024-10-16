pub mod mtc {
    use std::sync::{ Arc, Mutex };
    use std::thread::{ self, JoinHandle };

    /// Struct for a shared counter.
    pub struct SharedCounter {
        counter: u64,
    }

    impl SharedCounter {
        fn increment(&mut self) {
            self.counter += 1;
        }
    }

    /// Struct for a multi-threaded counter.
    pub struct MultiThreadCounter {
        workers: Vec<JoinHandle<()>>,
        shared_counter: Arc<Mutex<SharedCounter>>,
    }

    impl MultiThreadCounter {
        /// Creates a new MultiThreadCounter.
        ///
        /// # Example
        /// 
        /// ```
        /// use concurrency::mtc::MultiThreadCounter;
        /// 
        /// let mut counter = MultiThreadCounter::new(); // Create new counter.
        /// ```
        pub fn new() -> Self {
            MultiThreadCounter {
                workers: Vec::new(),
                shared_counter: Arc::new(Mutex::new(SharedCounter { counter: 0 })),
            }
        }

        /// Submits a number of increments to the counter.
        ///
        /// # Example
        /// 
        /// ```
        /// use concurrency::mtc::MultiThreadCounter;
        /// 
        /// let mut counter = MultiThreadCounter::new();
        /// counter.submit(100); // Submit 100 increments.
        /// ```
        pub fn submit(&mut self, amt: u64) {
            let shared_counter = Arc::clone(&self.shared_counter);
            let handle = thread::spawn(move || {
                for _ in 0..amt {
                    let mut counter = shared_counter.lock().unwrap();
                    counter.increment();
                }
            });
            self.workers.push(handle);
        }

        /// Gets the final value of the counter after all threads have completed.
        ///
        /// # Example
        /// 
        /// ```
        /// use concurrency::mtc::MultiThreadCounter;
        /// 
        /// let mut counter = MultiThreadCounter::new();
        /// counter.submit(100);
        /// let final_count = counter.get(); // Get the final result.
        /// ```
        pub fn get(&mut self) -> u64 {
            for handle in self.workers.drain(..) {
                handle.join().unwrap();
            }
            self.shared_counter.lock().unwrap().counter
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_multithread_counter_x10() {
            for _ in 0..10 {
                let mut counter = MultiThreadCounter::new();
                for _ in 0..100 {
                    counter.submit(10);
                }
                let final_count = counter.get();
                assert_eq!(final_count, 1000);
            }
        }
    }
}
