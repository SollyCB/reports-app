use std::{
    io, 
    thread,
    sync::{mpsc, Arc, Mutex},
};

#[derive(Debug)]
pub enum PoolCreationError {
    PoolSizeIsZero,
    FailedToSpawnWorker,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}


impl Worker {
    pub fn spawn(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> io::Result<Worker> {
        let builder = thread::Builder::new();

        let thread = builder.spawn(move || { loop {
            let job = receiver.lock().expect("failed to acquire lock in worker-{id}").recv();

            match job {
                Ok(job) => {
                    println!("Worker-{id} got job, executing");
                    job();
                },
                Err(_) => { 
                    println!("Worker-{id} shutting down");
                    break;
                },
            };

        }})?;

        Ok( Worker { id, thread: Some(thread) } )
    }
}

impl ThreadPool {
    /// Creates a new ThreadPool where size is the number of threads in the pool. 
    ///
    /// Returns Ok(ThreadPool) or Err if the size == 0
    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size == 0 { return Err(PoolCreationError::PoolSizeIsZero) }

        let mut workers = Vec::with_capacity(size);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size {
            match Worker::spawn(id, Arc::clone(&receiver)) {
                Ok(worker) => workers.push(worker),
                Err(_) => return Err(PoolCreationError::FailedToSpawnWorker),
            };
        }

        Ok(ThreadPool{ workers, sender: Some(sender) })
    }

    pub fn execute<F>(&self, f: F)
    where F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in &mut self.workers {
            println!("terminating worker-{}", worker.id);

            if let Some(thread) = worker.thread.take() { thread.join().unwrap() }
        }
    }
}
