use std::sync::{mpsc, Arc, Mutex};
use std::thread;

use log::{debug, info};

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    tx: Option<mpsc::Sender<Job>>,
    workers: Vec<Worker>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0, "ThreadPool must have positive size");

        let (tx, rx) = match mpsc::channel() {
            (tx, rx) => (tx, Arc::new(Mutex::new(rx))),
        };

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&rx)));
        }
        ThreadPool {
            tx: Some(tx),
            workers,
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.tx
            .as_ref()
            .expect("tx doesn't exist")
            .send(Box::new(f))
            .expect("send job fail");
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        debug!("dropping threadpool");
        // let a = &self.tx;
        drop(self.tx.take());

        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().expect("join error");
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, rx: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            match rx.lock().expect("acquire lock fail").recv() {
                Ok(job) => {
                    debug!("worker {} get job, start working", id);
                    job();
                }
                Err(_) => {
                    debug!("channel disconnected, closing worker {}", id);
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
