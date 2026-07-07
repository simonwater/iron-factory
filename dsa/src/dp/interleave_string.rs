/// [97. 交错字符串](https://leetcode.cn/problems/interleaving-string/description/)
pub struct Solution;

impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        let (i1, i2, i3) = (s1.len(), s2.len(), s3.len());
        if i1 + i2 != i3 {
            return false;
        }
        let mut memo: Vec<Vec<Option<bool>>> = vec![vec![None; i2 + 1]; i1 + 1];
        Self::dfs(
            i1,
            i2,
            s1.as_bytes(),
            s2.as_bytes(),
            s3.as_bytes(),
            &mut memo,
        )
    }

    fn dfs(
        i1: usize,
        i2: usize,
        s1: &[u8],
        s2: &[u8],
        s3: &[u8],
        memo: &mut Vec<Vec<Option<bool>>>,
    ) -> bool {
        if i1 == 0 && i2 == 0 {
            return true;
        }
        if let Some(ans) = memo[i1][i2] {
            return ans;
        }
        let i3 = i1 + i2;
        let mut ans = false;
        if i1 > 0 && i3 > 0 && s1[i1 - 1] == s3[i3 - 1] {
            ans = Self::dfs(i1 - 1, i2, s1, s2, s3, memo);
        }
        if !ans && i2 > 0 && i3 > 0 && s2[i2 - 1] == s3[i3 - 1] {
            ans = Self::dfs(i1, i2 - 1, s1, s2, s3, memo);
        }
        memo[i1][i2] = Some(ans);
        ans
    }

    pub fn dp(s1: String, s2: String, s3: String) -> bool {
        let (n1, n2, n3) = (s1.len(), s2.len(), s3.len());
        if n1 + n2 != n3 {
            return false;
        }
        let (s1, s2, s3) = (s1.as_bytes(), s2.as_bytes(), s3.as_bytes());
        let mut dp = vec![vec![false; n2 + 1]; n1 + 1];
        dp[0][0] = true;
        for i in 1..=n1 {
            if s1[i - 1] == s3[i - 1] {
                dp[i][0] = true;
            } else {
                break;
            }
        }
        for j in 1..=n2 {
            if s2[j - 1] == s3[j - 1] {
                dp[0][j] = true;
            } else {
                break;
            }
        }
        for i in 1..=n1 {
            for j in 1..=n2 {
                let k = i + j;
                dp[i][j] = (s1[i - 1] == s3[k - 1] && dp[i - 1][j])
                    || (s2[j - 1] == s3[k - 1] && dp[i][j - 1]);
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
        let text1 = String::from("aabcc");
        let text2 = String::from("dbbca");
        let text3 = String::from("aadbbcbcac");
        assert_eq!(Solution::is_interleave(text1, text2, text3), true);

        let text1 = String::from("aabcc");
        let text2 = String::from("dbbca");
        let text3 = String::from("aadbbbaccc");
        assert_eq!(Solution::is_interleave(text1, text2, text3), false);

        let text1 = String::from("");
        let text2 = String::from("");
        let text3 = String::from("");
        assert_eq!(Solution::is_interleave(text1, text2, text3), true);
    }

    #[test]
    fn test2() {
        let text1 = String::from("aabcc");
        let text2 = String::from("dbbca");
        let text3 = String::from("aadbbcbcac");
        assert_eq!(Solution::dp(text1, text2, text3), true);

        let text1 = String::from("aabcc");
        let text2 = String::from("dbbca");
        let text3 = String::from("aadbbbaccc");
        assert_eq!(Solution::dp(text1, text2, text3), false);

        let text1 = String::from("aabc");
        let text2 = String::from("abad");
        let text3 = String::from("aabadabc");
        assert_eq!(Solution::dp(text1, text2, text3), true);

        let text1 = String::from("");
        let text2 = String::from("");
        let text3 = String::from("");
        assert_eq!(Solution::dp(text1, text2, text3), true);
    }
}
