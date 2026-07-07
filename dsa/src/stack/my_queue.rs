/// [232. 用栈实现队列](https://leetcode.cn/problems/implement-queue-using-stacks/description/)
pub struct MyQueue {
    stack1: Vec<i32>,
    stack2: Vec<i32>,
}

impl MyQueue {
    pub fn new() -> Self {
        MyQueue {
            stack1: Vec::new(),
            stack2: Vec::new(),
        }
    }

    pub fn push(&mut self, x: i32) {
        self.stack1.push(x);
    }

    pub fn pop(&mut self) -> i32 {
        if self.empty() {
            return -1;
        }
        if self.stack2.is_empty() {
            while let Some(val) = self.stack1.pop() {
                self.stack2.push(val);
            }
        }
        self.stack2.pop().unwrap()
    }

    /// 此方法实现有两种选择：1.执行栈数据迁移，减轻pop的压力。2.保持不可变
    pub fn peek(&self) -> i32 {
        if self.empty() {
            return -1;
        }
        if self.stack2.is_empty() {
            *self.stack1.first().unwrap_or(&-1)
        } else {
            *self.stack2.last().unwrap_or(&-1)
        }
    }

    pub fn empty(&self) -> bool {
        self.stack1.is_empty() && self.stack2.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let mut q = MyQueue::new();
        q.push(1);
        q.push(2);
        assert_eq!(q.peek(), 1);
        assert_eq!(q.pop(), 1);
        assert_eq!(q.empty(), false);
    }
}
