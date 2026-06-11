/// [322. 零钱兑换](https://leetcode.cn/problems/coin-change/)
/// 完全背包，硬币可重复选择，数量无限
struct Solution;

impl Solution {
    /// dp[i]:构成金额i所需的最少硬币个数
    pub fn coin_change(mut coins: Vec<i32>, amount: i32) -> i32 {
        coins.sort_unstable();
        let amount = amount as usize;
        let mut dp = vec![i32::MAX / 2; amount + 1];
        dp[0] = 0;
        for val in 1..=amount {
            for &coin in coins.iter() {
                let coin_usize = coin as usize;
                if coin_usize > val {
                    break;
                }
                let cnt = dp[val - coin_usize] + 1;
                if cnt < dp[val] {
                    dp[val] = cnt;
                }
            }
        }
        if dp[amount] == i32::MAX / 2 {
            -1
        } else {
            dp[amount]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
