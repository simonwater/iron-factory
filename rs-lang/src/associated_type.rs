// 关联类型，trait和实现之间1对1
pub mod m1 {
    pub struct Counter {
        cnt: u32,
    }

    impl Counter {
        pub fn new() -> Self {
            Counter { cnt: 0 }
        }
    }

    impl Iterator for Counter {
        type Item = u32;
        fn next(&mut self) -> Option<Self::Item> {
            self.cnt += 1;
            Some(self.cnt)
        }
    }

    #[test]
    fn test() {
        let mut counter = Counter::new();
        println!("{}", counter.next().unwrap());
        println!("{}", counter.next().unwrap());
        println!("{}", counter.next().unwrap());
    }
}

// 范型，特征和实现之间 1对多
pub mod m2 {
    pub struct Counter {
        cnt1: u32,
        cnt2: i32,
    }

    impl Counter {
        pub fn new() -> Self {
            Counter { cnt1: 0, cnt2: 0 }
        }
    }

    pub trait MyIterator<T> {
        fn next(&mut self) -> Option<T>;
    }

    // impl 1:
    impl MyIterator<u32> for Counter {
        fn next(&mut self) -> Option<u32> {
            self.cnt1 += 1;
            Some(self.cnt1)
        }
    }

    // impl 2:
    impl MyIterator<i32> for Counter {
        fn next(&mut self) -> Option<i32> {
            self.cnt2 += 1;
            Some(self.cnt2)
        }
    }

    #[test]
    fn test() {
        let mut counter = Counter::new();
        println!(
            "{}",
            <Counter as MyIterator<u32>>::next(&mut counter).unwrap()
        );
        println!(
            "{}",
            <Counter as MyIterator<u32>>::next(&mut counter).unwrap()
        );
        println!(
            "{}",
            <Counter as MyIterator<u32>>::next(&mut counter).unwrap()
        );
    }
}
