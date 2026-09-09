//! mpsc::channel() 内部封装了底层的 OS 挂起与唤醒逻辑。当调用 rx.recv() 时，
//! 如果 Channel 里没有数据，当前 OS 线程会被操作系统自动挂起（不占用 CPU）；
//! 当生产者调用 tx.send() 时，内核会精准唤醒挂起在 rx 上的线程。

use std::collections::VecDeque;
use std::sync::Mutex;
use std::sync::mpsc::{self, Sender};

// 数据结构：列表数据 + 等待者 Channel 队列
pub struct SyncRedisList {
    pub data: Mutex<VecDeque<String>>,
    // 存放等待 Pop 的客户端 Sender 队列
    pub waiters: Mutex<VecDeque<Sender<String>>>,
}

impl SyncRedisList {
    // PUSH 操作：如果有等待者，直接将数据发给 Sender 唤醒线程
    pub fn push(&self, val: String) {
        let mut waiters = self.waiters.lock().unwrap();
        if let Some(tx) = waiters.pop_front() {
            // 直接将数据发送给阻塞等待的线程，并唤醒它
            let _ = tx.send(val);
            return;
        }

        // 没有等待者，正常存入队列
        let mut guard = self.data.lock().unwrap();
        guard.push_back(val);
    }

    // BPOP 操作：如果为空，创建一个 Channel 并阻塞等待 recv()
    pub fn bpop(&self) -> String {
        let mut guard = self.data.lock().unwrap();

        // 1. 如果有数据，直接弹出返回
        if let Some(val) = guard.pop_front() {
            return val;
        }

        // 2. 如果为空，创建一个一次性的 Channel
        let (tx, rx) = mpsc::channel();
        self.waiters.lock().unwrap().push_back(tx);

        // 释放 data 锁，避免阻塞其他线程
        drop(guard);

        // 3. 核心：recv() 会挂起当前 OS 线程，直到 PUSH 端 send() 数据进来
        rx.recv().unwrap()
    }
}
