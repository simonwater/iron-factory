//! 系统级线程和 async 的组合使用
#[cfg(test)]
mod tests {
    use std::thread;
    use std::time::Duration;

    #[test]
    fn msg_pass2() {
        let (tx, mut rx) = trpl::channel();
        thread::spawn(move || {
            for i in 1..=10 {
                println!("send msg: {i}");
                tx.send(i).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });
        trpl::block_on(async {
            while let Some(messaage) = rx.recv().await {
                println!("receive msg: {messaage}");
            }
        });
    }
}
