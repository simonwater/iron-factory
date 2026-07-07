/// [155. 最小栈](https://leetcode.cn/problems/min-stack/)
pub struct MinStack {
    stack: Vec<(i32, i32)>,
}

impl MinStack {
    pub fn new() -> Self {
        Self {
            stack: Vec::with_capacity(128),
        }
    }

    pub fn push(&mut self, val: i32) {
        let cur_min = match self.stack.last() {
            Some(&(_, prev_min)) => val.min(prev_min),
            None => val,
        };
        self.stack.push((val, cur_min));
    }

    pub fn pop(&mut self) {
        self.stack.pop();
    }

    pub fn top(&self) -> i32 {
        self.stack.last().unwrap().0
    }

    pub fn get_min(&self) -> i32 {
        self.stack.last().unwrap().1
    }
}
