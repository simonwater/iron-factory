//! 一个极简的运行时执行器

use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

/// 极简执行器的 waker 什么都没做，空的。future能正常工作依赖的是下面的loop不断轮询去判断状态
pub fn block_on<F: Future>(mut future: F) -> F::Output {
    let mut future = unsafe { Pin::new_unchecked(&mut future) };

    fn noop_raw_waker() -> RawWaker {
        fn no_op(_: *const ()) {}
        fn clone(_: *const ()) -> RawWaker {
            noop_raw_waker()
        }
        let vtable = &RawWakerVTable::new(clone, no_op, no_op, no_op);
        RawWaker::new(std::ptr::null(), vtable)
    }
    let waker = unsafe { Waker::from_raw(noop_raw_waker()) };
    let mut cx = Context::from_waker(&waker);

    loop {
        match future.as_mut().poll(&mut cx) {
            Poll::Ready(value) => return value,
            Poll::Pending => {
                std::thread::yield_now();
            }
        }
    }
}

/// 由轮询线程A执行
pub struct FlagFuture {
    flag: Arc<AtomicBool>,
    waker: Arc<Mutex<Option<Waker>>>,
}

impl FlagFuture {
    pub fn new(flag: Arc<AtomicBool>, waker: Arc<Mutex<Option<Waker>>>) -> Self {
        Self { flag, waker }
    }
}

impl Future for FlagFuture {
    type Output = &'static str;

    /// Pending状态必须在首尾进行双重检测。通常运行时的实现不同于此处的极简执行器轮询检测，检测状态是由
    /// future 内部调用 wake发起的。
    /// > - A线程进来准备设置waker，此时waker为None
    /// > - A尚未设置好 waker，B线程已经成功更新状态，调用waker的wake方法，但waker为空，没有调用到
    /// > - A线程设置好了waker继续往后走，此时状态已经更新，但没有二次检测，直接返回了pending
    /// > - wake 方法再也没有时机被调用，future 会被一直挂起永远不会返回Ready
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.flag.load(Ordering::Acquire) {
            return Poll::Ready("Flag changed!");
        }

        *self.waker.lock().unwrap() = Some(cx.waker().clone());

        if self.flag.load(Ordering::Acquire) {
            return Poll::Ready("Flag changed!");
        } else {
            return Poll::Pending;
        }
    }
}

/// 由工作线程 B 执行
pub fn set_flag(flag: &AtomicBool, waker_slot: &Mutex<Option<Waker>>) {
    flag.store(true, Ordering::Release);
    if let Some(w) = waker_slot.lock().unwrap().take() {
        w.wake();
    }
}

#[cfg(test)]
mod executor_tests {
    use std::thread;
    use std::time::Duration;

    use super::super::future_simple_impl::CountdownFuture;
    use super::*;

    #[test]
    fn test_simple() {
        let result = block_on(async {
            println!("Hello from our mini executor!");
            123
        });
        println!("Got: {result}");
    }

    #[test]
    fn test_counter() {
        let result = block_on(async {
            println!("start counter");
            let counter = CountdownFuture::new(10);
            let ans = counter.await;
            println!("end counter: {ans}");
            "success!"
        });
        println!("Got: {result}");
    }

    #[test]
    fn test_flag() {
        let flag = Arc::new(AtomicBool::new(false));
        let waker: Arc<Mutex<Option<Waker>>> = Arc::new(Mutex::new(None));
        let future = FlagFuture::new(Arc::clone(&flag), Arc::clone(&waker));
        thread::spawn(move || {
            thread::sleep(Duration::from_secs(3));
            set_flag(&flag, &waker);
        });

        let result = block_on(future);
        println!("Got: {result}");
    }
}
