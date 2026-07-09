/// # [121. 买卖股票的最佳时机](https://leetcode.cn/problems/best-time-to-buy-and-sell-stock/)
/// > 买卖1次
pub struct Solution;

impl Solution {
    ///  **前缀和 贪心**
    ///
    /// 记录过去的最低价格作为买入价，枚举每一天的价格作为卖出价，同时收集最大利润
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let (mut prev, mut ans) = (prices[0], 0);
        for price in prices {
            ans = std::cmp::max(ans, price - prev);
            prev = std::cmp::min(prev, price);
        }
        ans
    }
}

pub struct Solution2;

impl Solution2 {
    ///  **动态规划**
    ///
    /// 状态定义：
    /// - cash: 第i天结束时持有现金的**最大累积收益**
    /// - stock: 第i天结束时持有股票的**最大累积收益**
    /// 需要特别注意只能买卖1次，所以开始买入持有股票时，手里初始启动资金为0，不是以前通过交易累积到的 `cash`。
    /// 如果使用以前累积到的 `cash`，就变成了允许无限次交易
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let (mut cash, mut stock) = (0, -prices[0]);
        for p in prices {
            (cash, stock) = (cash.max(stock + p), stock.max(-p));
        }
        cash
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let prices = vec![7, 1, 5, 3, 6, 4];
        assert_eq!(Solution::max_profit(prices), 5);

        let prices = vec![1, 2, 3, 4, 5];
        assert_eq!(Solution::max_profit(prices), 4);

        let prices = vec![7, 6, 4, 3, 1];
        assert_eq!(Solution::max_profit(prices), 0);
    }

    #[test]
    fn test2() {}
}
