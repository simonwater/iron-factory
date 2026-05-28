/// [155. 最小栈](https://leetcode.cn/problems/min-stack/)
struct MinStack {
    stack: Vec<(i32, i32)>,
}

impl MinStack {
    fn new() -> Self {
        Self {
            stack: Vec::with_capacity(128),
        }
    }

    fn push(&mut self, val: i32) {
        let cur_min = match self.stack.last() {
            Some(&(_, prev_min)) => val.min(prev_min),
            None => val,
        };
        self.stack.push((val, cur_min));
    }

    fn pop(&mut self) {
        self.stack.pop();
    }

    fn top(&self) -> i32 {
        self.stack.last().unwrap().0
    }

    fn get_min(&self) -> i32 {
        self.stack.last().unwrap().1
    }
}
