//！控制权交还给运行时
use std::thread;
use std::time::Duration;

pub fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{name}' ran for {ms}ms");
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 通过 sleep 把控制权交还给运行时
    #[test]
    fn test() {
        let one_ms = Duration::from_millis(1);
        trpl::block_on(async {
            let a = async {
                println!("'a' started.");
                slow("a", 30);
                trpl::sleep(one_ms).await;
                slow("a", 10);
                trpl::sleep(one_ms).await;
                slow("a", 20);
                trpl::sleep(one_ms).await;
                println!("'a' finished.");
            };

            let b = async {
                println!("'b' started.");
                slow("b", 75);
                trpl::sleep(one_ms).await;
                slow("b", 10);
                trpl::sleep(one_ms).await;
                slow("b", 15);
                trpl::sleep(one_ms).await;
                slow("b", 350);
                trpl::sleep(one_ms).await;
                println!("'b' finished.");
            };

            println!("start...");
            trpl::select(a, b).await;
        });
    }

    /// 通过 yield_now 把控制权交还给运行时
    #[test]
    fn test2() {
        trpl::block_on(async {
            let a = async {
                println!("'a' started.");
                slow("a", 30);
                trpl::yield_now().await;
                slow("a", 10);
                trpl::yield_now().await;
                slow("a", 20);
                trpl::yield_now().await;
                println!("'a' finished.");
            };

            let b = async {
                println!("'b' started.");
                slow("b", 75);
                trpl::yield_now().await;
                slow("b", 10);
                trpl::yield_now().await;
                slow("b", 15);
                trpl::yield_now().await;
                slow("b", 350);
                trpl::yield_now().await;
                println!("'b' finished.");
            };

            println!("start...");
            trpl::select(a, b).await;
        });
    }
}
