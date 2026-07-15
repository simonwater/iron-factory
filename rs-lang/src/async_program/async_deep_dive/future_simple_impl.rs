use std::{
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
    thread,
    time::Duration,
};

type Job = Box<dyn FnOnce() + Send + 'static>;

/// 延时执行器，Future的简单实现。可以延迟特定时间后执行闭包
pub struct Delay {
    completed: Arc<Mutex<bool>>,
    job: Option<Job>,
    waker_stored: Arc<Mutex<Option<Waker>>>,
    started: bool,
    duration: Duration,
}

impl Delay {
    pub fn new(job: Job, duration: Duration) -> Self {
        Delay {
            completed: Arc::new(Mutex::new(false)),
            waker_stored: Arc::new(Mutex::new(None)),
            job: Some(job),
            started: false,
            duration,
        }
    }
}

impl Future for Delay {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("poll");
        if *self.completed.lock().unwrap() {
            return Poll::Ready(());
        }

        *self.waker_stored.lock().unwrap() = Some(cx.waker().clone());
        if !self.started {
            self.started = true;
            let duration = self.duration;
            let completed = Arc::clone(&self.completed);
            let waker = Arc::clone(&self.waker_stored);
            let job = self.job.take();

            thread::spawn(move || {
                thread::sleep(duration);
                if let Some(f) = job {
                    f();
                }
                *completed.lock().unwrap() = true;

                waker.lock().unwrap().take().unwrap().wake();
            });
        }

        if *self.completed.lock().unwrap() {
            return Poll::Ready(());
        }

        Poll::Pending
    }
}

pub struct CountdownFuture {
    count: u32,
}

impl CountdownFuture {
    pub fn new(count: u32) -> Self {
        Self { count }
    }
}

impl Future for CountdownFuture {
    type Output = &'static str;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.count == 0 {
            return Poll::Ready("Liftoff!");
        } else {
            println!("count:{}", self.count);
            self.count -= 1;
            cx.waker().wake_by_ref();
            return Poll::Pending;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delay() {
        println!("start test");
        trpl::block_on(async {
            println!("start delay");
            let job = Box::new(|| {
                println!("run job!");
            });
            let delay = Delay::new(job, Duration::from_secs(3));
            delay.await;
            println!("end delay");
        });
        println!("end test");
    }

    #[test]
    fn test_counter() {
        trpl::block_on(async {
            println!("start counter");
            let counter = CountdownFuture::new(10);
            let ans = counter.await;
            println!("end counter: {ans}");
        });
    }
}
