use std::thread;

pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
}

impl ThreadPool {
    /// Create a new ThreadPool
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero
    pub fn new(size: usize) -> ThreadPool {
        // panics if size <= 0
        // for prod code, should return Result
        assert!(size > 0);

        let mut threads = Vec::with_capacity(size);

        for _ in 0..size {
          // TODO: create threads
        }

        ThreadPool {threads}
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}
