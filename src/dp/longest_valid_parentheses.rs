/// # [32. 最长有效括号](https://leetcode.cn/problems/longest-valid-parentheses/)
///

struct Solution;
impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let n = s.len();
        if n <= 1 {
            return 0;
        }
        let s_bytes = s.as_bytes();
        let mut dp = vec![0; n + 1];
        let mut ans = 0usize;
        for i in 1..n {
            let c = s_bytes[i];
            let cnt = i + 1; // 对应dp数组的索引（表示前cnt个字符）
            if c == b')' {
                if s_bytes[i - 1] == b'(' {
                    dp[cnt] = dp[cnt - 2] + 2;
                } else {
                    let prev = dp[cnt - 1];
                    if i - prev > 0 && s_bytes[i - prev - 1] == b'(' {
                        dp[cnt] = 2 + prev + dp[i - prev - 1];
                    }
                }
            }
            ans = ans.max(dp[cnt]);
        }
        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
