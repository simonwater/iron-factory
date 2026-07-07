/// [188. 买卖股票的最佳时机 IV](https://leetcode.cn/problems/best-time-to-buy-and-sell-stock-iv/description/)
pub struct Solution;

impl Solution {
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        let k = k as usize;
        let mut buys = vec![-prices[0]; k + 1];
        let mut sells = vec![0; k + 1];
        for price in prices {
            for j in 1..=k {
                buys[j] = std::cmp::max(buys[j], sells[j - 1] - price);
                sells[j] = std::cmp::max(sells[j], buys[j] + price);
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
