/// [215. 数组中的第K个最大元素](https://leetcode.cn/problems/kth-largest-element-in-an-array/)
pub struct Solution;

use rand::{Rng, rngs::ThreadRng};
impl Solution {
    // 快速选择
    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        if k <= 0 || k as usize > nums.len() {
            unreachable!();
        }
        let n = nums.len();
        let k = n - (k as usize);
        let mut rng = rand::rng(); // 只构造一次
        Self::qselect(&mut nums, 0, n - 1, k, &mut rng)
    }

    fn qselect(nums: &mut [i32], start: usize, end: usize, k: usize, rng: &mut ThreadRng) -> i32 {
        if start >= end {
            return nums[start];
        }
        let idx: usize = rng.random_range(start..=end);
        let pivot = nums[idx];
        let mut left = start;
        let mut i = start;
        let mut right = end;
        while i <= right {
            if nums[i] == pivot {
                i += 1;
            } else if nums[i] < pivot {
                nums.swap(left, i);
                left += 1;
                i += 1;
            } else {
                nums.swap(i, right);
                right -= 1;
            }
        }
        if k < left {
            Self::qselect(nums, start, left - 1, k, rng)
        } else if k > right {
            Self::qselect(nums, right + 1, end, k, rng)
        } else {
            pivot
        }
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

        let nums = vec![1];
        let k = 1;
        let ans = 1;
        assert_eq!(Solution::find_kth_largest(nums, k), ans);
    }
}
