/// # [416. 分割等和子集](https://leetcode.cn/problems/partition-equal-subset-sum/)
///
/// 状态定义：`dp[amt][i]` 表示nums中前i个是否能构成amt
struct Solution;

impl Solution {
    // 
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let mut target = nums.iter().sum::<i32>() as usize;
        if target % 2 == 1 {
            return false;
        }
        target = target / 2;
        let n = nums.len();
        let mut dp = vec![vec![false; n + 1]; target + 1];
        for val in dp[0].iter_mut() {
            *val = true;
        }
        for amt in 1..=target {
            for i in 1..=n {
                let num = nums[i - 1] as usize;
                if num > amt {
                    dp[amt][i] = dp[amt][i - 1];
                } else {
                    dp[amt][i] = dp[amt][i - 1] || dp[amt - num][i - 1];
                }
            }
        }
        dp[target][n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![1, 5, 11, 5];
        assert_eq!(Solution::can_partition(nums), true);

        let nums = vec![1, 2, 3, 5];
        assert_eq!(Solution::can_partition(nums), false);
    }
}
