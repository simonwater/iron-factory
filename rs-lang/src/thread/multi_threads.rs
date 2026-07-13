#[cfg(test)]
mod test {
    use std::thread;
    use std::time::Duration;

    #[test]
    fn t1() {
        thread::spawn(|| {
            for i in 1..=10 {
                println!("number {i} from child thread");
                thread::sleep(Duration::from_millis(1));
            }
        });
        for i in 1..=5 {
            println!("number {i} from main thread");
            thread::sleep(Duration::from_millis(1));
        }
    }

    #[test]
    fn t2() {
        let handler = thread::spawn(move || {
            for i in 1..=10 {
                println!("number {i} from child thread");
                thread::sleep(Duration::from_millis(1));
            }
        });
        for i in 1..=5 {
            println!("number {i} from main thread");
            thread::sleep(Duration::from_millis(1));
        }
        handler.join().unwrap();
    }

    #[test]
    fn test3() {
        let handle1 = thread::spawn(|| {
            for i in 1..=10 {
                println!("number {i} from the first thread");
                thread::sleep(Duration::from_millis(500));
            }
        });

        let handle2 = thread::spawn(|| {
            for i in 1..=5 {
                println!("number {i} from the second thread");
                thread::sleep(Duration::from_millis(500));
            }
        });
        handle2.join().unwrap();
        handle1.join().unwrap();
    }
}
