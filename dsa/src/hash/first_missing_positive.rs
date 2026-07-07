/// [41. 缺失的第一个正数](https://leetcode.cn/problems/first-missing-positive/)
pub struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let set: HashSet<i32> = HashSet::from_iter(nums.into_iter());
        let mut ans = 1;
        while set.contains(&ans) {
            ans += 1;
        }
        ans
    }

    pub fn first_missing_positive2(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        for i in 0..n {
            while nums[i] > 0 && nums[i] as usize <= n {
                let cur = nums[i] as usize;
                if cur == i + 1 || cur as i32 == nums[cur - 1] {
                    break;
                }
                nums.swap(i, cur - 1);
            }
        }
        for (i, &num) in nums.iter().enumerate() {
            if num as usize != i + 1 {
                return i as i32 + 1;
            }
        }
        n as i32 + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![1, 2, 0];
        let ans = 3;
        assert_eq!(Solution::first_missing_positive(nums), ans);

        let nums = vec![3, 4, -1, 1];
        let ans = 2;
        assert_eq!(Solution::first_missing_positive(nums), ans);

        let nums = vec![7, 8, 9, 11, 12];
        let ans = 1;
        assert_eq!(Solution::first_missing_positive(nums), ans);

        let nums = vec![1, 1];
        let ans = 2;
        assert_eq!(Solution::first_missing_positive(nums), ans);
    }

    #[test]
    fn test2() {
        let nums = vec![1, 2, 0];
        let ans = 3;
        assert_eq!(Solution::first_missing_positive2(nums), ans);

        let nums = vec![3, 4, -1, 1];
        let ans = 2;
        assert_eq!(Solution::first_missing_positive2(nums), ans);

        let nums = vec![7, 8, 9, 11, 12];
        let ans = 1;
        assert_eq!(Solution::first_missing_positive2(nums), ans);

        let nums = vec![1, 1];
        let ans = 2;
        assert_eq!(Solution::first_missing_positive2(nums), ans);
    }
}
