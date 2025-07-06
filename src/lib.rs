use std::sync::{
    mpsc::{self, Receiver},
    Arc, Mutex,
};

#[allow(dead_code)]
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

#[allow(dead_code)]
struct Worker {
    id: usize,
    thread: std::thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        let thread = std::thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {id} is waiting for a job");

            job();
        });

        Worker { id, thread }
    }
}

impl ThreadPool {
    /// Creates a new ThreadPool
    ///
    /// The `size` parameter specifies the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if `size` is less than or equal to 0.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0, "ThreadPool size must be greater than 0");

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}
