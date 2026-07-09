/// # [188. 买卖股票的最佳时机 IV](https://leetcode.cn/problems/best-time-to-buy-and-sell-stock-iv/)
/// > 可以完成 k 笔交易(买、卖各 k 次)
///
/// ## 状态定义
/// - `buys[j] (1 <= j <= k)`：恰好正在进行第 j 次买入/持仓时的最大收益。
/// - `sells[j] (1 <= j <= k)`：恰好完成了第 j 次卖出/平仓时的最大收益。
///
/// ## 状态转移方程
/// 对于第 j 次交易（1 <= j <= k）
/// 1. 第 j 次买入状态：`buys[j] = max(buys[j], sells[j - 1] - p)`
///     - 物理含义：今天更新第 j 次持仓状态，要么保持昨天第 j 次持仓，要么拿着第 j - 1 次卖出后赚到的净利润 `sells[j - 1]`，在今天发起第 j 次买入    
/// 2. 第 j 次卖出状态：`sells[j] = max(sells[j], buys[j] + p)`
///     - 物理含义：今天更新第 j 次平仓状态，要么保持昨天第 j 次平仓，要么把第 j 次持仓的股票在今天以价格 p 卖出
pub struct Solution;

impl Solution {
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        let k = k as usize;
        let mut buys = vec![i32::MIN; k + 1];
        let mut sells = vec![0; k + 1];
        for price in prices {
            for j in 1..=k {
                buys[j] = buys[j].max(sells[j - 1] - price);
                sells[j] = sells[j].max(buys[j] + price);
            }
        }

        sells[k]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let prices = vec![2, 4, 1];
        assert_eq!(Solution::max_profit(2, prices), 2);

        let prices = vec![3, 2, 6, 5, 0, 3];
        assert_eq!(Solution::max_profit(2, prices), 7);
    }

    #[test]
    fn test2() {}
}
