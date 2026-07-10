/// 线程间通过共享内存来进行通信
#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::{sync::Mutex, thread};

    #[test]
    fn test_single() {
        let m = Mutex::new(5);
        {
            // 调用lock() 会阻塞当前线程，直到拥有锁为止
            // 如果另一个线程拥有锁，并且那个线程 panic 了，则 lock 调用会失败
            let mut num = m.lock().unwrap();
            println!("value is: {}", num);
            *num = 10;
            println!("value is: {}", num);
        }
        println!("m = {m:?}");
    }

    #[test]
    fn test_multi() {
        let counter = Arc::new(Mutex::new(0));
        let mut handles = Vec::with_capacity(10);
        let data = Arc::new(vec![1, 2, 3]);
        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            let data1 = data.clone();
            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();
                *num += 1;
            });
            println!("{}", data1[0]);
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("Result: {}", *counter.lock().unwrap());
    }
}
