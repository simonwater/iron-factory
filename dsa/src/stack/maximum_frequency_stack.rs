//! [895. 最大频率栈](https://leetcode.cn/problems/maximum-frequency-stack/)
//!
//! 频率最高的先弹出，相同频率的，后进入的先弹出。
//! 维护一个频率的桶数组，最后一个桶里装着频率最高的数。高频率的数在低频率桶里同时存放，对应着数被弹出且频率
//! 减少后，在低频率桶里同样还能找到。哈希表维护每个数出现的频率，频率增加时去对应的频率桶里进行追加

use std::collections::HashMap;

pub struct FreqStack {
    num_freq: HashMap<i32, i32>,
    freq_stacks: Vec<Vec<i32>>,
}

impl FreqStack {
    pub fn new() -> Self {
        Self {
            num_freq: HashMap::with_capacity(256),
            freq_stacks: Vec::with_capacity(256),
        }
    }

    pub fn push(&mut self, val: i32) {
        let freq = self.num_freq.entry(val).or_insert(0);
        *freq += 1;
        let freq_usize = *freq as usize;
        if self.freq_stacks.len() < freq_usize {
            self.freq_stacks.push(Vec::with_capacity(32));
        }
        self.freq_stacks[freq_usize - 1].push(val);
    }

    pub fn pop(&mut self) -> i32 {
        if let Some(stack) = self.freq_stacks.last_mut() {
            if let Some(num) = stack.pop() {
                if stack.is_empty() {
                    self.freq_stacks.pop();
                }
                let freq = self.num_freq.get_mut(&num);
                *freq.unwrap() -= 1;
                return num;
            }
        }

        unreachable!()
    }
}
