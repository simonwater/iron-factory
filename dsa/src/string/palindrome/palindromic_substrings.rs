/// [647. 回文子串](https://leetcode.cn/problems/palindromic-substrings/)
pub struct Solution;

impl Solution {
    /// 中心扩散法
    pub fn count_substrings(s: String) -> i32 {
        let mut ans = 0;
        let s_bytes = s.as_bytes();
        for i in 0..s_bytes.len() {
            ans += Self::calc(s_bytes, i, i);
            ans += Self::calc(s_bytes, i, i + 1);
        }
        ans
    }

    pub fn count_substrings2(s: String) -> i32 {
        let s_bytes = s.as_bytes();
        (0..s_bytes.len())
            .map(|i| Self::calc(s_bytes, i, i) + Self::calc(s_bytes, i, i + 1))
            .sum()
    }

    fn calc(s_bytes: &[u8], mut i: usize, mut j: usize) -> i32 {
        let mut cnt = 0;
        while j < s_bytes.len() {
            if s_bytes[i] == s_bytes[j] {
                cnt += 1;
                if i == 0 {
                    break;
                }
                i -= 1;
                j += 1;
            } else {
                break;
            }
        }
        cnt
    }
}

pub struct Solution2;
/// 动态规划
impl Solution2 {
    pub fn count_substrings(s: String) -> i32 {
        let mut ans = 0;
        let s_bytes = s.as_bytes();
        let n = s_bytes.len();
        // dp[i][j] 表示字符串s[i..=j]是否是回文
        let mut dp = vec![vec![false; n]; n];
        for j in 0..n {
            // 先循环j，也就是尾部
            dp[j][j] = true;
            ans += 1;
            for i in 0..j {
                if s_bytes[i] == s_bytes[j] {
                    dp[i][j] = if j <= i + 1 { true } else { dp[i + 1][j - 1] };
                    if dp[i][j] {
                        ans += 1;
                    }
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
