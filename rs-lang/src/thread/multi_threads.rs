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
}
