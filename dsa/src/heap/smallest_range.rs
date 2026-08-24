//! [632. 最小区间](https://leetcode.cn/problems/smallest-range-covering-elements-from-k-lists/)
//!
use std::{cmp::Reverse, collections::binary_heap::BinaryHeap};

pub struct Solution;

impl Solution {
    pub fn smallest_range(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let k = nums.len();
        let mut heap = BinaryHeap::with_capacity(k);
        let mut max = nums[0][0];
        for i in 0..k {
            let num = nums[i][0];
            heap.push(Reverse((num, i, 0)));
            if num > max {
                max = num;
            }
        }
        let (mut start, _, _) = heap.peek().unwrap().0;
        let mut end = max;
        let mut len = i32::MAX;
        while let Some(Reverse((min, r, c))) = heap.pop() {
            let cur_len = max - min + 1;
            if cur_len < len {
                len = cur_len;
                start = min;
                end = max;
            }
            if cur_len == 1 || c == nums[r].len() - 1 {
                break;
            }
            let next = nums[r][c + 1];
            if next > max {
                max = next;
            }
            heap.push(Reverse((next, r, c + 1)));
        }

        vec![start, end]
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
