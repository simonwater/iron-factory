/// 通过通道（类似队列）进行线程间的消息传递
#[cfg(test)]
mod test {
    use std::sync::mpsc;
    use std::sync::mpsc::TryRecvError;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_recv() {
        let (tx, rx) = mpsc::channel();
        println!("spwan new thread.");
        thread::spawn(move || {
            let msgs = vec!["value1", "value2", "value3"];
            for msg in msgs {
                // 子线程发送消息
                tx.send(msg).unwrap();
                thread::sleep(Duration::from_millis(500));
            }
            thread::sleep(Duration::from_millis(5000));
            println!("child thread is closed!");
        });

        // 主线程接收消息
        loop {
            // 同步接口调用recv时如果有消息则无论发送端是否还存活，都先取得 Ok(msg);
            // 如果没有消息，但发送端端还存活，则会被阻塞；
            // 如果没有消息且所有发送端都被销毁，则得到Err
            match rx.recv() {
                Ok(msg) => println!("message from child: {}", msg),
                Err(_) => {
                    // 消息都消耗完，且所有tx都销毁得到 Err
                    println!("all senders are closed!");
                    break;
                }
            }
        }
    }

    #[test]
    fn test_recv2() {
        let (tx, rx) = mpsc::channel();
        println!("spwan new thread.");
        thread::spawn(move || {
            let msgs = vec!["value1", "value2", "value3"];
            for msg in msgs {
                tx.send(msg).unwrap();
                thread::sleep(Duration::from_millis(500));
            }
            thread::sleep(Duration::from_millis(5000));
            tx.send("bye!").unwrap();
            println!("child thread is closed!");
        });

        for msg in rx {
            println!("message from child: {}", msg)
        }
        println!("all senders are closed!");
    }

    #[test]
    fn test_try_recv() {
        let (tx, rx) = mpsc::channel();
        println!("spwan new thread.");
        thread::spawn(move || {
            let msgs = vec!["value1", "value2", "value3"];
            for msg in msgs {
                tx.send(msg).unwrap();
                thread::sleep(Duration::from_millis(500));
            }
            // 消息发完先sleep，此时接收端会先持续消耗消息，消息消耗完再轮询时会持续收到 Empty
            thread::sleep(Duration::from_millis(5000));
            println!("child thread is closed!");
            // 线程销毁后接收端轮询时收到 Disconnected
        });

        loop {
            match rx.try_recv() {
                Ok(msg) => println!("message from child: {}", msg),
                Err(TryRecvError::Empty) => {
                    println!("no message now, please wait a moment!");
                    // 模拟去处理其他事务，到时间后回来继续轮询时，即便发送端都已经销毁，只要channel中还有消息就会继续得到 Ok(msg)
                    thread::sleep(Duration::from_secs(3));
                }
                Err(TryRecvError::Disconnected) => {
                    // 消息都消耗完，且所有发送端都销毁，得到 Disconnected
                    println!("all senders are closed!");
                    break;
                }
            }
        }
    }
}
