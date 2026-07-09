/// # [122. 买卖股票的最佳时机 II](https://leetcode.cn/problems/best-time-to-buy-and-sell-stock-ii/)
/// > 可以无限次交易
pub struct Solution;

impl Solution {
    /// **动态规划**
    ///
    /// 区别于 121. 这里可以无限次交易，所以第 i 天结束时，进行买入股票操作后的收益是在以前累积到的现金 `cash` 基础上进行的.
    /// 本质区别在于决定“今天买入股票”的那个刹那，手头上能动用的启动资金来自哪里。
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let (mut cash, mut stock) = (0, -prices[0]);
        for price in prices {
            (cash, stock) = (cash.max(stock + price), stock.max(cash - price));
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
        assert_eq!(Solution::max_profit(prices), 7);

        let prices = vec![1, 2, 3, 4, 5];
        assert_eq!(Solution::max_profit(prices), 4);

        let prices = vec![7, 6, 4, 3, 1];
        assert_eq!(Solution::max_profit(prices), 0);
    }

    #[test]
    fn test2() {}
}
