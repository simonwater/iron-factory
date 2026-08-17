//! 模拟异步任务被编译为Future状态机的过程

use std::pin::Pin;
use std::task::{Context, Poll};

/// src
pub async fn example(min_len: usize) -> String {
    let content = async_read_file("foo.txt").await;
    if content.len() < min_len {
        content + &async_read_file("bar.txt").await
    } else {
        content
    }
}

async fn async_read_file(_name: &str) -> String {
    String::from("testtest")
}

/// target
struct StartState {
    min_len: usize,
}

struct WaitingOnFooTxtState {
    min_len: usize,
    foo_txt_future: Pin<Box<dyn Future<Output = String>>>,
}

struct WaitingOnBarTxtState {
    content: String,
    bar_txt_future: Pin<Box<dyn Future<Output = String>>>,
}

struct EndState {}

enum ExampleStateMachine {
    Start(StartState),
    WaitingOnFooTxt(WaitingOnFooTxtState),
    WaitingOnBarTxt(WaitingOnBarTxtState),
    End(EndState),
}

pub fn example_main(min_len: usize) -> ExampleFuture {
    ExampleFuture {
        state: ExampleStateMachine::Start(StartState { min_len }),
    }
}

pub struct ExampleFuture {
    state: ExampleStateMachine,
}

impl Future for ExampleFuture {
    type Output = String; // `example` 的返回类型

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = unsafe { self.as_mut().get_unchecked_mut() };
        loop {
            match &mut this.state {
                // TODO: 处理 pinning
                ExampleStateMachine::Start(state) => {
                    // 来自 `example` 函数体
                    let foo_txt_future = async_read_file("foo.txt");
                    // `.await` 运算符
                    let state = WaitingOnFooTxtState {
                        min_len: state.min_len,
                        foo_txt_future: Box::pin(foo_txt_future),
                    };
                    this.state = ExampleStateMachine::WaitingOnFooTxt(state); // 通过 loop 向前推进
                }
                ExampleStateMachine::WaitingOnFooTxt(state) => {
                    match state.foo_txt_future.as_mut().poll(cx) {
                        Poll::Pending => return Poll::Pending,
                        Poll::Ready(content) => {
                            // 来自 `example` 函数体
                            if content.len() < state.min_len {
                                let bar_txt_future = async_read_file("bar.txt");
                                // `.await` 运算符
                                let state = WaitingOnBarTxtState {
                                    content,
                                    bar_txt_future: Box::pin(bar_txt_future),
                                };
                                this.state = ExampleStateMachine::WaitingOnBarTxt(state); // 向前推进
                            } else {
                                this.state = ExampleStateMachine::End(EndState {});
                                return Poll::Ready(content);
                            }
                        }
                    }
                }
                ExampleStateMachine::WaitingOnBarTxt(state) => {
                    match state.bar_txt_future.as_mut().poll(cx) {
                        Poll::Pending => return Poll::Pending,
                        Poll::Ready(bar_txt) => {
                            let content = match std::mem::replace(
                                &mut this.state,
                                ExampleStateMachine::End(EndState {}),
                            ) {
                                ExampleStateMachine::WaitingOnBarTxt(WaitingOnBarTxtState {
                                    content,
                                    ..
                                }) => content,
                                _ => unreachable!(),
                            };
                            return Poll::Ready(content + &bar_txt);
                        }
                    }
                }
                ExampleStateMachine::End(_state) => {
                    panic!("poll called after Poll::Ready was returned");
                }
            }
        }
    }
}
