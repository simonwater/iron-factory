//! # 简单线程池实现。
//!
//! 线程池在初始化时便根据容量创建好多个工作线程，主线程和工作线程间用 `mpsc` 的 `channel` 进行通信。
//! 主线程通过 `Sender` 发送工作任务，子线程内部通过 `Receiver` 等待并接收任务。由于 `mpsc` 中的 `Receiver`
//! 无法在多个线程间进行clone，所以通过 `Arc<Mutex<Receiver>>` 来进行线程间共享一个 `Receiver`，
//! 注意此过程：获取锁 -> 等待或接收任务 -> 执行任务
//! ```
//! let job = receiver.lock().unwrap().recv().unwrap();
//! job();
//! ```
//! 通道中有多个任务时，一个线程获取锁后很快便能接收到一个任务，代码中锁是临时变量，线程得到任务的语句结
//! 束后锁便释放，执行任务时不妨碍其他线程继续获取锁接收任务。通道中没有任务时，某个线程获取锁后被 `recv()`
//! 阻塞，锁没被释放，虽然其他线程被锁阻塞，但通道中没有任务，这样的阻塞没有任何问题。当有任务来临，获取
//! 锁的线程能快速取得任务并把锁释放，继续让别的线程竞争锁接收任务。

use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

type Job = Box<dyn FnOnce() + Send + 'static>;
struct Worker {
    id: usize,
    thread: Option<JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Self {
        let thread: JoinHandle<()> = thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv();
                match message {
                    Ok(job) => {
                        println!("Worker {id} got a job; executing.");
                        job();
                    }
                    Err(_) => {
                        println!("Worker {id} disconnected; shutting down.");
                        break;
                    }
                }
            }
        });
        Self {
            id,
            thread: Some(thread),
        }
    }
}

pub struct ThreadPool {
    sender: Option<Sender<Job>>,
    workers: Vec<Worker>,
}

impl ThreadPool {
    pub fn new() -> Self {
        Self::with_capacity(5)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(capacity);
        for i in 1..=capacity {
            let rx = Arc::clone(&receiver);
            let worker = Worker::new(i, rx);
            workers.push(worker);
        }

        Self {
            sender: Some(sender),
            workers,
        }
    }

    pub fn execute<F>(&mut self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender.as_ref().unwrap().send(Box::new(f)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use std::{thread, time::Duration};

    use super::*;
    #[test]
    fn test() {
        let mut pool = ThreadPool::new();
        let task_cnt = 20;
        for tid in 1..=task_cnt {
            pool.execute(move || {
                println!("begin task id: {}", tid);
                thread::sleep(Duration::from_secs(2));
                println!("end task id: {}", tid);
            });
        }
    }
}
