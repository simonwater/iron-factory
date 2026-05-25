/// [215. 数组中的第K个最大元素](https://leetcode.cn/problems/kth-largest-element-in-an-array/)
struct Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    // 小顶堆
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut heap = BinaryHeap::with_capacity(k);
        for num in nums {
            let item = Reverse(num);
            if heap.len() < k {
                heap.push(item);
            } else {
                if let Some(mut top) = heap.peek_mut() {
                    if item < *top {
                        *top = item;
                    }
                }
            }
        }
        heap.pop().unwrap_or(Reverse(-1)).0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![3, 2, 1, 5, 6, 4];
        let k = 2;
        let ans = 5;
        assert_eq!(Solution::find_kth_largest(nums, k), ans);

        let nums = vec![3, 2, 3, 1, 2, 4, 5, 5, 6];
        let k = 4;
        let ans = 4;
        assert_eq!(Solution::find_kth_largest(nums, k), ans);
    }
}
