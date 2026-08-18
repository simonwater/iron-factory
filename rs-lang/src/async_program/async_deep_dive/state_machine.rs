//! 模拟异步任务被编译为Future状态机的过程
//!
//! 异步任务会被编译器编译成划分为多个阶段的任务状态机，然后交给运行时运行。状态机实现了 Futrue，同各个阶段的子任务、
//! 子子任务对应的Future共同构成了一个Future的树，运行时每次轮询执行任务时，会把唤醒器waker传递给任务，任务通过层层
//! 调用子任务把waker一直传递到叶子节点，最终叶子future会将waker注册到底层事件驱动器中。当运行时执行任务时，如果任务
//! 当前阶段对应的子任务处于完成状态，状态机就向前推进到后续阶段。如果子任务处于等待状态，运行时的线程不会等着任务完成，
//! 而是轮询执行其他任务或者挂起自身。当任务的当前阶段对应的I/O操作已经就绪时，运行时的事件驱动器会收到操作系统的通知，
//! 由最底层的叶子任务注册到事件驱动器中的的waker会被触发调用，用来唤醒或者通知运行时继续轮询当前任务，以使任务继续往前
//! 推进到后续阶段，直至完成。

use std::pin::Pin;
use std::task::{Context, Poll};
use std::unreachable;

/// 异步任务
pub async fn example(min_len: usize) -> String {
    let content = async_read_file("foo.txt").await; // stage1
    if content.len() < min_len {
        content + &async_read_file("bar.txt").await // stage2
    } else {
        content
    }
}

async fn async_read_file(_name: &str) -> String {
    String::from("testtest")
}

/// 异步任务对应的状态机
enum ExampleStateMachine {
    Start {
        min_len: usize,
    },
    WaitingOnStage1 {
        min_len: usize,
        stage1_future: Pin<Box<dyn Future<Output = String>>>, // 子futrue
    },
    WaitingOnStage2 {
        content: String,
        stage2_future: Pin<Box<dyn Future<Output = String>>>, // 子futrue
    },
    End,
}

pub fn example_main(min_len: usize) -> ExampleFuture {
    ExampleFuture {
        state: ExampleStateMachine::Start { min_len },
    }
}

pub struct ExampleFuture {
    state: ExampleStateMachine,
    // todo: 记录waker
}

impl Future for ExampleFuture {
    type Output = String; // `example` 的返回类型

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = unsafe { self.as_mut().get_unchecked_mut() };
        loop {
            match &mut this.state {
                ExampleStateMachine::Start { min_len } => {
                    // 来自 `example` 函数体
                    let stage1_future = async_read_file("foo.txt");
                    // `.await` 运算符
                    this.state = ExampleStateMachine::WaitingOnStage1 {
                        min_len: *min_len,
                        stage1_future: Box::pin(stage1_future),
                    }; // 切换状态，并通过 loop 向前推进
                }
                ExampleStateMachine::WaitingOnStage1 {
                    min_len,
                    stage1_future,
                } => {
                    // 向子future传递waker
                    match stage1_future.as_mut().poll(cx) {
                        Poll::Pending => return Poll::Pending,
                        Poll::Ready(content) => {
                            // 来自 `example` 函数体
                            if content.len() < *min_len {
                                let stage2_future = async_read_file("bar.txt");
                                // `.await` 运算符
                                this.state = ExampleStateMachine::WaitingOnStage2 {
                                    content,
                                    stage2_future: Box::pin(stage2_future),
                                }; // 切换状态，向前推进
                            } else {
                                this.state = ExampleStateMachine::End;
                                return Poll::Ready(content);
                            }
                        }
                    }
                }
                ExampleStateMachine::WaitingOnStage2 { stage2_future, .. } => {
                    // 向子futrue传递waker
                    match stage2_future.as_mut().poll(cx) {
                        Poll::Pending => return Poll::Pending,
                        Poll::Ready(bar_txt) => {
                            // 移出content
                            let content = match std::mem::replace(
                                &mut this.state,
                                ExampleStateMachine::End,
                            ) {
                                ExampleStateMachine::WaitingOnStage2 { content, .. } => content,
                                _ => unreachable!(),
                            };
                            return Poll::Ready(content + &bar_txt);
                        }
                    }
                }
                ExampleStateMachine::End => {
                    panic!("poll called after Poll::Ready was returned");
                }
            }
        }
    }
}
