pub struct ThreadPool {
    size: u32,
    threads: Vec<>
}

impl ThreadPool {
    pub fn new(size: u32) -> ThreadPool {
        assert!(size > 0, "ThreadPool must have positive size");
        let threads = Vec::with_capacity(size);
        for _ in 0..size {

        }
        ThreadPool { size, threads }
    }
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}
