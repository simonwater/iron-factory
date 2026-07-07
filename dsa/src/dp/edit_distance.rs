/// [72. 编辑距离](https://leetcode.cn/problems/edit-distance/description/)
pub struct Solution;

impl Solution {
    pub fn min_distance(s1: String, s2: String) -> i32 {
        let (n1, n2) = (s1.len(), s2.len());
        let (s_bytes1, s_byte2) = (s1.as_bytes(), s2.as_bytes());
        let mut dp = vec![vec![0; n2 + 1]; n1 + 1];
        for i in 0..=n1 {
            dp[i][0] = i as i32;
        }
        for j in 0..=n2 {
            dp[0][j] = j as i32;
        }
        for i in 1..=n1 {
            for j in 1..=n2 {
                if s_bytes1[i - 1] == s_byte2[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1];
                } else {
                    dp[i][j] = 1 + std::cmp::min(
                        dp[i - 1][j - 1],
                        std::cmp::min(dp[i - 1][j], dp[i][j - 1]),
                    );
                }
            }
        }
        dp[n1][n2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let text1 = String::from("horse");
        let text2 = String::from("ros");
        assert_eq!(Solution::min_distance(text1, text2), 3);

        let text1 = String::from("intention");
        let text2 = String::from("execution");
        assert_eq!(Solution::min_distance(text1, text2), 5);
    }

    #[test]
    fn test2() {}
}
