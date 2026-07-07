/// [120. 三角形最小路径和](https://leetcode.cn/problems/triangle/)
pub struct Solution;

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let n = triangle.len();
        let mut dp = triangle[n - 1].clone();
        for i in (0..n - 1).rev() {
            for j in 0..=i {
                dp[j] = triangle[i][j] + std::cmp::min(dp[j], dp[j + 1]);
            }
        }
        dp[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let triangle = vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]];
        assert_eq!(Solution::minimum_total(triangle), 11);

        let triangle = vec![vec![-10]];
        assert_eq!(Solution::minimum_total(triangle), -10);
    }

    #[test]
    fn test2() {}
}
