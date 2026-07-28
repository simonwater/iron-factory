//! [115. 不同的子序列](https://leetcode.cn/problems/distinct-subsequences/)
//!
//! 动态规划 dp[i][j] 表示 s 前 i 个字符中出现 t 的前 j 个字符的次数。
//!

pub struct Solution;

impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let s_len = s.len();
        let t_len = t.len();
        if s_len < t_len {
            return 0;
        }
        let s = s.as_bytes();
        let t = t.as_bytes();
        let mut dp = vec![vec![0; t_len + 1]; s_len + 1];
        dp[0][0] = 1;
        for i in 1..=s_len {
            dp[i][0] = 1;
            for j in 1..=t_len {
                dp[i][j] = dp[i - 1][j];
                if s[i - 1] == t[j - 1] {
                    dp[i][j] += dp[i - 1][j - 1];
                }
            }
        }

        dp[s_len][t_len]
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
