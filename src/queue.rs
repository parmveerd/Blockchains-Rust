use std::sync::mpsc;
use std::thread;

pub trait Task {
    type Output: Send;
    fn run(&self) -> Option<Self::Output>;
}

pub struct WorkQueue<TaskType: 'static + Task + Send> {
    send_tasks: Option<spmc::Sender<TaskType>>, // Option because it will be set to None to close the queue
    recv_tasks: spmc::Receiver<TaskType>,
    //send_output: mpsc::Sender<TaskType::Output>, // not need in the struct: each worker will have its own clone.
    recv_output: mpsc::Receiver<TaskType::Output>,
    workers: Vec<thread::JoinHandle<()>>,
}

impl<TaskType: 'static + Task + Send> WorkQueue<TaskType> {
    pub fn new(n_workers: usize) -> WorkQueue<TaskType> {
        let (send_tasks, recv_tasks) = spmc::channel();
        let (send_output, recv_output) = mpsc::channel();

        let mut workers = Vec::with_capacity(n_workers);
        for _ in 0..n_workers {
            let recv_tasks_clone = recv_tasks.clone();
            let send_output_clone = send_output.clone();
            let worker_handle = thread::spawn(move || {
                WorkQueue::<TaskType>::run(recv_tasks_clone, send_output_clone);
            });
            workers.push(worker_handle);
        }

        WorkQueue {
            send_tasks: Some(send_tasks),
            recv_tasks,
            recv_output,
            workers,
        }
    }

    fn run(recv_tasks: spmc::Receiver<TaskType>, send_output: mpsc::Sender<TaskType::Output>) {
        loop {
            let task_result = recv_tasks.recv(); 

            if let Ok(task) = task_result {
                if let Some(output) = task.run() {
                    send_output.send(output).expect("Failed to send task output.");
                }
            } else {
                break;
            }
        }
    }

    pub fn enqueue(&mut self, t: TaskType) -> Result<(), spmc::SendError<TaskType>> {
        match &mut self.send_tasks {
            Some(sender) => sender.send(t),
            None => Err(spmc::SendError(t)),
        }
    }

    // Helper methods that let you receive results in various ways
    pub fn iter(&mut self) -> mpsc::Iter<TaskType::Output> {
        self.recv_output.iter()
    }
    pub fn recv(&mut self) -> TaskType::Output {
        self.recv_output
            .recv()
            .expect("I have been shutdown incorrectly")
    }
    pub fn try_recv(&mut self) -> Result<TaskType::Output, mpsc::TryRecvError> {
        self.recv_output.try_recv()
    }
    pub fn recv_timeout(
        &self,
        timeout: std::time::Duration,
    ) -> Result<TaskType::Output, mpsc::RecvTimeoutError> {
        self.recv_output.recv_timeout(timeout)
    }

    pub fn shutdown(&mut self) {
        // Destroy the spmc::Sender so everybody knows no more tasks are incoming;
        // drain any pending tasks in the queue; wait for each worker thread to finish.
        // HINT: Vec.drain(..)
        if let Some(sender) = self.send_tasks.take() {
            drop(sender);

            // Drain any pending tasks in the queue
            while let Ok(_) = self.recv_tasks.recv() {
                // Get rid of the received task without running it.
            }

            // Wait for each worker thread to finish.
            for worker in self.workers.drain(..) {
                worker.join().expect("Failed to join worker thread.");
            }
        }
    }
}

impl<TaskType: 'static + Task + Send> Drop for WorkQueue<TaskType> {
    fn drop(&mut self) {
        // "Finalisation in destructors" pattern: https://rust-unofficial.github.io/patterns/idioms/dtor-finally.html
        match self.send_tasks {
            None => {} // already shut down
            Some(_) => self.shutdown(),
        }
    }
}
