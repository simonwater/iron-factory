use std::ops::Deref;

pub struct MyBox<T>(T);

impl<T> MyBox<T> {
    pub fn new(value: T) -> Self {
        Self(value)
    }
}

/// 通过实现Deref trait 可以使智能指针拥有类似于引用的行为
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("Bye!!");
    }
}

pub fn hello(s: &str) {
    println!("Hello {s}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let x = 5;
        let y = MyBox::new(x);
        let _z = y.max(111);
        assert_eq!(x, 5);
        assert_eq!(x, *y);
        assert_eq!(x, *(y.deref()));

        let s = MyBox::new(String::from("ok"));
        // 解引用转换(deref coercion)可以将某个实现了Deref trait的类型的引用转换为另一个类型的引用
        // 1. &MуBox<String> 转换为 &String
        // 2. &String 转换为 &str
        hello(&s);
    }
}
