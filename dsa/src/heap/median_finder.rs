/// [295. 数据流的中位数](https://leetcode.cn/problems/find-median-from-data-stream/)
pub struct MedianFinder {
    left_max: BinaryHeap<i32>,
    right_min: BinaryHeap<Reverse<i32>>,
}

use std::{cmp::Reverse, collections::BinaryHeap};
impl MedianFinder {
    pub fn new() -> Self {
        Self {
            left_max: BinaryHeap::with_capacity(128),
            right_min: BinaryHeap::with_capacity(128),
        }
    }

    pub fn add_num(&mut self, num: i32) {
        if self.left_max.len() == self.right_min.len() {
            match self.right_min.peek() {
                Some(&Reverse(val)) if num > val => {
                    let Reverse(val) = self.right_min.pop().unwrap();
                    self.left_max.push(val);
                    self.right_min.push(Reverse(num));
                }
                _ => self.left_max.push(num),
            }
        } else {
            match self.left_max.peek() {
                Some(&val) if num < val => {
                    self.left_max.pop();
                    self.right_min.push(Reverse(val));
                    self.left_max.push(num);
                }
                _ => self.right_min.push(Reverse(num)),
            }
        }
    }

    pub fn find_median(&self) -> f64 {
        if self.left_max.len() > self.right_min.len() {
            return *self.left_max.peek().unwrap() as f64;
        } else {
            let left_val = *self.left_max.peek().unwrap();
            let Reverse(right_val) = self.right_min.peek().unwrap();
            return (left_val + right_val) as f64 / 2.0;
        }
    }
}
