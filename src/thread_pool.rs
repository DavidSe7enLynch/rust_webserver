use log::{debug, error};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

type Job = Box<dyn FnOnce() -> Result<(), String> + Send + 'static>;

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

    pub fn execute<F>(&self, f: F) -> Result<(), String>
    where
        F: FnOnce() -> Result<(), String> + Send + 'static,
    {
        self.tx
            .as_ref()
            .expect("tx doesn't exist")
            .send(Box::new(f))
            .map_err(|e| format!("send job err: {:#?}", e))?;
        Ok(())
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        debug!("dropping threadpool");
        drop(self.tx.take());

        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().expect("join err");
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
        Worker {
            _id: id,
            thread: Some(thread::spawn(move || Worker::work(id, rx))),
        }
    }

    fn work(id: usize, rx: Arc<Mutex<mpsc::Receiver<Job>>>) {
        loop {
            match rx.lock().expect("acquire lock err").recv() {
                Ok(job) => {
                    debug!("worker {id} get job, start working");
                    job().unwrap_or_else(|e| error!("worker {id} executes job err: {e}"));
                }
                Err(_) => {
                    debug!("channel disconnected, closing worker {id}");
                    break;
                }
            }
        }
    }
}
