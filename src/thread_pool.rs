use log::debug;
use std::error::Error;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    tx: Option<mpsc::Sender<Job>>,
    workers: Vec<Worker>,
}

impl ThreadPool {
    pub fn build(size: usize) -> Result<ThreadPool, String> {
        if size <= 0 {
            return Err("ThreadPool must have positive size".to_string());
        }

        let (tx, rx) = match mpsc::channel() {
            (tx, rx) => (tx, Arc::new(Mutex::new(rx))),
        };

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&rx)));
        }
        Ok(ThreadPool {
            tx: Some(tx),
            workers,
        })
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
    _id: usize,
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
            _id: id,
            thread: Some(thread),
        }
    }
}
