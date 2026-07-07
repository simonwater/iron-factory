/// # [64. 最小路径和](https://leetcode.cn/problems/minimum-path-sum/)
///

pub struct Solution;

impl Solution {
    //
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid[0].len();
        let mut dp = vec![0; n];
        for (r, row) in grid.iter().enumerate() {
            for (c, &val) in row.iter().enumerate() {
                if r == 0 && c == 0 {
                    dp[c] = val;
                } else if r == 0 {
                    dp[c] = dp[c - 1] + val;
                } else if c == 0 {
                    dp[c] = dp[c] + val;
                } else {
                    dp[c] = dp[c].min(dp[c - 1]) + val;
                }
            }
        }
        dp[n - 1]
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
