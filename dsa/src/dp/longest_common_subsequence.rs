/// [1143. 最长公共子序列](https://leetcode.cn/problems/longest-common-subsequence/)

pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
    let (n1, n2) = (text1.len(), text2.len());
    let mut memo = vec![vec![None; n2]; n1];
    return dfs(text1.as_bytes(), text2.as_bytes(), n1, n2, &mut memo);
}

pub fn dfs(
    s_bytes1: &[u8],
    s_bytes2: &[u8],
    i1: usize,
    i2: usize,
    memo: &mut Vec<Vec<Option<i32>>>,
) -> i32 {
    if i1 == 0 || i2 == 0 {
        return 0;
    }
    if let Some(ans) = memo[i1 - 1][i2 - 1] {
        return ans;
    }
    let ans = if s_bytes1[i1 - 1] == s_bytes2[i2 - 1] {
        dfs(s_bytes1, s_bytes2, i1 - 1, i2 - 1, memo) + 1
    } else {
        let v1 = dfs(s_bytes1, s_bytes2, i1 - 1, i2, memo);
        let v2 = dfs(s_bytes1, s_bytes2, i1, i2 - 1, memo);
        std::cmp::max(v1, v2)
    };
    memo[i1 - 1][i2 - 1] = Some(ans);
    ans
}

pub fn dp(text1: String, text2: String) -> i32 {
    let (n1, n2) = (text1.len(), text2.len());
    let mut dp = vec![vec![0; n2 + 1]; n1 + 1];
    let s1 = text1.as_bytes();
    let s2 = text2.as_bytes();
    for i in 0..n1 {
        for j in 0..n2 {
            if s1[i] == s2[j] {
                dp[i + 1][j + 1] = dp[i][j] + 1;
            } else {
                dp[i + 1][j + 1] = std::cmp::max(dp[i + 1][j], dp[i][j + 1])
            }
        }
    }
    return dp[n1][n2];
}

/// 空间优化，二维数组压缩成一维。
/// 对二维dp数组做更新时，更新值只取决于当前值左、上、左上三个位置的值
pub struct Solution;
impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let n = text2.len();
        let mut dp = vec![0; n + 1];
        for &c1 in text1.as_bytes() {
            let mut left_top = 0;
            for (i, &c2) in text2.as_bytes().iter().enumerate() {
                let old = dp[i + 1];
                if c1 == c2 {
                    dp[i + 1] = left_top + 1;
                } else {
                    dp[i + 1] = dp[i].max(dp[i + 1]);
                }
                left_top = old;
            }
        }
        dp[n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dfs_test() {
        let text1 = String::from("abcdef");
        let text2 = String::from("ace");
        assert_eq!(longest_common_subsequence(text1, text2), 3);

        let text1 = String::from("abc");
        let text2 = String::from("abc");
        assert_eq!(longest_common_subsequence(text1, text2), 3);

        let text1 = String::from("abc");
        let text2 = String::from("def");
        assert_eq!(longest_common_subsequence(text1, text2), 0);
    }

    #[test]
    fn dp_test() {
        let text1 = String::from("abcdef");
        let text2 = String::from("ace");
        assert_eq!(dp(text1, text2), 3);

        let text1 = String::from("abc");
        let text2 = String::from("abc");
        assert_eq!(dp(text1, text2), 3);

        let text1 = String::from("abc");
        let text2 = String::from("def");
        assert_eq!(dp(text1, text2), 0);
    }
}
