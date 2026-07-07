/// # [322. 零钱兑换](https://leetcode.cn/problems/coin-change/)
/// 求组成target的最小数量。关键条件：
///     1. 没有重复硬币
///     2. 每种硬币数量无限可无限重复选择（完全背包）
///
/// 状态定义：`dp[i]`:构成金额i所需的最少硬币个数
///
/// 本题两层循环里“外层跑金额，内层跑数字”，本质上是不断枚举构成当前金额的最后一个数字，由于每次都是从头枚举，
/// 事实上是在枚举“排列”，比如由[1,2]构成3，那么(1,2)和(2,1)都会被枚举到，他们是相同组合，事实上是重复选择了，但因为
/// 题目要求的是最少数量，而不是总数，所以对最终结果没有影响

pub struct Solution;

impl Solution {
    //
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
    #[test]
    fn test1() {}
}
