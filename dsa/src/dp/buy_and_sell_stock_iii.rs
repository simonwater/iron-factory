/// [123. 买卖股票的最佳时机 III](https://leetcode.cn/problems/best-time-to-buy-and-sell-stock-iii/description/)
pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let n = prices.len();
        let (mut buy1, mut sell1, mut buy2, mut sell2) = (-prices[0], 0, -prices[0], 0);
        for i in 1..n {
            let price = prices[i];
            let new_buy1 = std::cmp::max(buy1, -price);
            let new_sell1 = std::cmp::max(sell1, buy1 + price);
            let new_buy2 = std::cmp::max(buy2, sell1 - price);
            let new_sell2 = std::cmp::max(sell2, buy2 + price);
            (buy1, sell1, buy2, sell2) = (new_buy1, new_sell1, new_buy2, new_sell2);
        }
        sell2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let prices = vec![3, 3, 5, 0, 0, 3, 1, 4];
        assert_eq!(Solution::max_profit(prices), 6);

        let prices = vec![1, 2, 3, 4, 5];
        assert_eq!(Solution::max_profit(prices), 4);

        let prices = vec![7, 6, 4, 3, 1];
        assert_eq!(Solution::max_profit(prices), 0);

        let prices = vec![1];
        assert_eq!(Solution::max_profit(prices), 0);
    }

    #[test]
    fn test2() {}
}
