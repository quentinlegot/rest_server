use std::vec::Vec;
use std::thread;
use std::sync::{mpsc, Arc, Mutex};

/// ThreadPool struct to manage multiple tasks in parallel.
/// Totally inspired by the threadpool example from the official rust tutorial / book, see [here](https://doc.rust-lang.org/book/ch20-02-multithreaded.html), everything is explained in the book.
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {

    pub fn new(size: usize) -> Self {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for _ in 0..size {
            workers.push(Worker::new(Arc::clone(&receiver)));
        }
        Self {
            workers,
            sender
        }
    }

    pub fn execute<F>(&self, f: F)
    where 
        F: FnOnce() + Send + 'static + Sync, {
            let job = Box::new(f);
            self.sender.send(Message::NewJob(job)).unwrap();
        }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }
        println!("Shutting down all workers");
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Self {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();
            match message {
                Message::NewJob(job) => {
                    // println!("Worker {} got a job to do", id);
                    job();
                }
                Message::Terminate => {
                    break;
                }
            }
        });

        Self {
            thread: Some(thread)
        }
    }
}

enum Message {
    NewJob(Job),
    Terminate,
}