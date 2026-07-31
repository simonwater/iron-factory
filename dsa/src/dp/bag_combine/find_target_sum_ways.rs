//! [494. 目标和](https://leetcode.cn/problems/target-sum/)
//!
//! 问题转化成求和为target的组合的总数，**求组合总数**.
//! 状态定义：dp[i][v]: 前 i 个数中和为 v 的组合个数

pub struct Solution;

impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, mut target: i32) -> i32 {
        let sum = nums.iter().sum::<i32>();
        target = sum - target;
        if target < 0 || target % 2 == 1 {
            return 0;
        }
        let target = (target / 2) as usize; // 从nums中选出和为target的组合数
        let n = nums.len();
        let mut dp = vec![vec![0; target + 1]; n + 1];
        dp[0][0] = 1;
        for i in 1..=n {
            let num = nums[i - 1] as usize;
            for v in 0..=target {
                dp[i][v] = dp[i - 1][v];
                if num <= v {
                    dp[i][v] += dp[i - 1][v - num]
                }
            }
        }
        dp[target][n]
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
