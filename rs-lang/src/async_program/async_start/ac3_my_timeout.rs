//! 组合 Future 构建新的异步抽象

use std::time::Duration;

use trpl::Either;

pub async fn timeout<F: Future>(f: F, max_time: Duration) -> Result<F::Output, Duration> {
    match trpl::select(f, trpl::sleep(max_time)).await {
        Either::Left(output) => Ok(output),
        Either::Right(_) => Err(max_time),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    #[test]
    fn test() {
        trpl::block_on(async {
            let slow_future = async {
                // std::thread::sleep(Duration::from_secs(3)); // wrong!
                trpl::sleep(Duration::from_secs(3)).await;
                "slow future ran for 3s"
            };

            println!("begin!");
            match timeout(slow_future, Duration::from_secs(4)).await {
                Ok(msg) => println!("Success with message: {msg}"),
                Err(duration) => {
                    println!("Timeout after {} seconds", duration.as_secs())
                }
            }
        });
    }
}
