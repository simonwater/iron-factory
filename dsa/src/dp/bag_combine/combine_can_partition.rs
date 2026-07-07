/// # [416. 分割等和子集](https://leetcode.cn/problems/partition-equal-subset-sum/)
///
/// 状态定义：`dp[i][amt]` 表示nums中前i个是否能构成amt
pub struct Solution;

impl Solution {
    //
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let mut target = nums.iter().sum::<i32>() as usize;
        if target % 2 == 1 {
            return false;
        }
        target = target / 2;
        let n = nums.len();
        let mut dp = vec![vec![false; target + 1]; n + 1];
        dp[0][0] = true;
        for i in 1..=n {
            dp[i][0] = true;
            let num = nums[i - 1] as usize;
            for amt in 1..=target {
                if num > amt {
                    dp[i][amt] = dp[i - 1][amt];
                } else {
                    dp[i][amt] = dp[i - 1][amt] || dp[i - 1][amt - num];
                }
            }
        }
        dp[n][target]
    }
}

/// # 状态压缩
/// 可以发现在计算 dp 的过程中，每一行的 dp 值都只与上一行的 dp 值有关，因此只需要一个一维数组即可将空间复杂度降到 O(target)。
/// 此时的转移方程为:<br>
/// `dp[j]=dp[j] ∣ dp[j−nums[i]]` <br>
/// 且需要注意的是第二层的循环我们需要从大到小计算，因为如果我们从小到大更新 dp 值，
/// 那么在计算 dp[j] 值的时候，dp[j−nums[i]] 已经是被更新过的状态，不再是上一行的 dp 值。
pub struct Solution2;

impl Solution2 {
    //
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let mut target = nums.iter().sum::<i32>() as usize;
        if target % 2 == 1 {
            return false;
        }
        target = target / 2;
        let n = nums.len();
        let mut dp = vec![false; target + 1];
        dp[0] = true;
        for i in 0..n {
            let num = nums[i] as usize;
            for amt in (num..=target).rev() {
                dp[amt] = dp[amt] || dp[amt - num];
            }
        }
        dp[target]
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
