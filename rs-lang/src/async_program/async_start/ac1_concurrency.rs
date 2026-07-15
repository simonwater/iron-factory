/// 1. 通过消息传递在future之间发送数据
/// 2. 一个 async 代码块内代码如何顺序执行
/// 3. 将所有权 move 进 async 块
/// 4. 合并多个 future
#[cfg(test)]
mod tests {
    use std::time::Duration;

    #[test]
    fn test1() {
        trpl::block_on(async {
            trpl::spawn_task(async {
                for i in 1..=10 {
                    println!("hi number {i} from the spawn task!");
                    trpl::sleep(Duration::from_millis(500)).await;
                }
            });

            for i in 1..=5 {
                println!("hi number {i} from the main task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });
    }

    #[test]
    fn test2() {
        trpl::block_on(async {
            let handle = trpl::spawn_task(async {
                for i in 1..=10 {
                    println!("hi number {i} from the spawn task!");
                    trpl::sleep(Duration::from_millis(500)).await;
                }
            });

            for i in 1..=5 {
                println!("hi number {i} from the main task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
            handle.await.unwrap()
        });
    }

    #[test]
    fn test3() {
        trpl::block_on(async {
            let future1 = async {
                for i in 1..=10 {
                    println!("hi number {i} from the first task!");
                    trpl::sleep(Duration::from_millis(500)).await;
                }
            };

            let future2 = async {
                for i in 1..=5 {
                    println!("hi number {i} from the second task!");
                    trpl::sleep(Duration::from_millis(500)).await;
                }
            };

            trpl::join(future1, future2).await;
        });
    }

    /// send和recv都在一个async块内，执行逻辑严格按代码顺序进行，所有send操作完成了才会到while recv块。
    /// 内部并没有并发进行send和recv
    #[test]
    fn msg_pass1() {
        trpl::block_on(async {
            let (tx, mut rx) = trpl::channel();

            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                println!("send: {val}");
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }

            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        });
    }

    /// 多个future之间并发执行 + 传递消息
    #[test]
    fn msg_pass2() {
        trpl::block_on(async {
            let (tx, mut rx) = trpl::channel();

            let tx1 = tx.clone();
            let tx_future1 = async move {
                let vals = vec![
                    String::from("hi"),
                    String::from("from"),
                    String::from("the"),
                    String::from("future"),
                ];

                for val in vals {
                    println!("tx1 send: {val}");
                    tx1.send(val).unwrap();
                    trpl::sleep(Duration::from_millis(500)).await;
                }
            };

            let rx_future = async {
                while let Some(value) = rx.recv().await {
                    println!("received '{value}'");
                }
            };

            let tx_future2 = async move {
                let vals = vec![
                    String::from("more"),
                    String::from("messages"),
                    String::from("for"),
                    String::from("you"),
                ];

                for val in vals {
                    println!("tx2 send: {val}");
                    tx.send(val).unwrap();
                    trpl::sleep(Duration::from_millis(500)).await;
                }
            };

            trpl::join!(tx_future1, tx_future2, rx_future);
        });
    }
}
