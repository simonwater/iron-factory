//! [135. 分发糖果](https://leetcode.cn/problems/candy/)
//!

pub struct Solution;

impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let n = ratings.len();
        let mut candies = vec![1; n];
        for i in 1..n {
            candies[i] = if ratings[i] > ratings[i - 1] {
                candies[i - 1] + 1
            } else {
                1
            };
        }
        for i in (0..n - 1).rev() {
            candies[i] = candies[i].max(if ratings[i] > ratings[i + 1] {
                candies[i + 1] + 1
            } else {
                1
            });
        }

        candies.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
