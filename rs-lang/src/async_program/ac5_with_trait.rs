#[cfg(test)]
mod tests {
    use std::pin::{Pin, pin};
    use std::time::Duration;

    #[test]
    fn msg_pass2() {
        trpl::block_on(async {
            let (tx, mut rx) = trpl::channel();

            let tx1 = tx.clone();
            let tx_future1 = pin!(async move {
                println!("tx1 send msg");
                tx1.send("tx1 msg").unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            });

            let rx_future = pin!(async {
                while let Some(value) = rx.recv().await {
                    println!("received '{value}'");
                }
            });

            let tx_future2 = pin!(async move {
                println!("tx2 send msg");
                tx.send("tx2 msg").unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            });

            println!("start...");
            let futrues: Vec<Pin<&mut dyn Future<Output = ()>>> =
                vec![tx_future1, tx_future2, rx_future];
            trpl::join_all(futrues).await;
        });
    }
}
