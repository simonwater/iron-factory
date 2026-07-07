use std::cell::RefCell;

pub trait Messenger {
    fn send(&self, s: &str);
}

pub struct MockMessenger {
    messages: RefCell<Vec<String>>,
}
impl MockMessenger {
    pub fn new() -> Self {
        Self {
            messages: RefCell::new(Vec::with_capacity(125)),
        }
    }
}

impl Messenger for MockMessenger {
    fn send(&self, s: &str) {
        let mut b1 = self.messages.borrow_mut();
        let mut b2 = self.messages.borrow_mut();
        // runtime panic!
        b1.push(String::from(s));
        b2.push(String::from(s));
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
