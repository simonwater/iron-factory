pub mod m1 {
    use std::fmt::Display;
    pub trait Abc {
        fn abc(&self) -> String;
    }

    // 为每个实现了Display的类型T都实现特征Abc
    // 类型T只要实现了Display，就会自动加上Abc特征的方法
    impl<T: Display> Abc for T {
        fn abc(&self) -> String {
            format!("abc-{}-abc", self)
        }
    }

    #[test]
    fn test() {
        println!("{}", 123.abc());
    }
}

pub mod m2 {
    use std::fmt::Display;

    // 超trait，Abc特征依赖了Display特征，
    // 所以实现Abc特征的类型必须依据实现了Display特征
    pub trait Abc: Display {
        fn abc(&self) -> String {
            format!("abc-{}-abc", self)
        }
    }

    impl Abc for i32 {}

    #[test]
    fn test() {
        println!("{}", 123.abc());
    }
}
