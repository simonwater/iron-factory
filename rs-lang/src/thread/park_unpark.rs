//! thread::park() 会挂起当前 OS 线程；其他线程持有该线程的 Thread 句柄（Handle）
//! 并调用 handle.unpark() 时，被挂起的线程会被操作系统唤醒。

use std::collections::VecDeque;
use std::sync::Mutex;
use std::thread::{self, Thread};

pub struct ParkRedisList {
    pub data: Mutex<VecDeque<String>>,
    // 记录正在等待的线程句柄
    pub waiting_threads: Mutex<VecDeque<Thread>>,
}

impl ParkRedisList {
    pub fn push(&self, val: String) {
        let mut guard = self.data.lock().unwrap();
        guard.push_front(val);

        // 唤醒一个等待的线程
        if let Some(thread_handle) = self.waiting_threads.lock().unwrap().pop_front() {
            thread_handle.unpark(); // 操作系统精准唤醒目标线程
        }
    }

    pub fn bpop(&self) -> String {
        loop {
            let mut guard = self.data.lock().unwrap();
            if let Some(val) = guard.pop_back() {
                return val;
            }

            // 数据为空，将当前线程 Handle 注册进去
            self.waiting_threads
                .lock()
                .unwrap()
                .push_back(thread::current());
            drop(guard);

            // 核心：挂起当前 OS 线程，等待 unpark 唤醒（配合 while/loop 防止虚假唤醒）
            thread::park();
        }
    }
}
