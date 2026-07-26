//! [132. 分割回文串 II](https://leetcode.cn/problems/palindrome-partitioning-ii/)
//!
//! 切割完成后所有子串都是回文串，用中心扩散法枚举所有回文子串的左右边界。

pub struct Solution;

impl Solution {
    pub fn min_cut(s: String) -> i32 {
        let n = s.len() as i32;
        let bytes = s.as_bytes();
        let mut dp: Vec<i32> = (0..n).collect();
        for i in 0..n {
            Self::check(bytes, i, i, &mut dp);
            Self::check(bytes, i, i + 1, &mut dp);
        }
        dp[s.len() - 1]
    }

    fn check(bytes: &[u8], mut left: i32, mut right: i32, dp: &mut [i32]) {
        let n = bytes.len() as i32;
        while left >= 0 && right < n {
            if bytes[left as usize] == bytes[right as usize] {
                let r_usize = right as usize;
                let l_usize = left as usize;
                if left == 0 {
                    dp[r_usize] = 0;
                } else {
                    dp[r_usize] = dp[r_usize].min(dp[l_usize - 1] + 1);
                }
                left -= 1;
                right += 1;
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
