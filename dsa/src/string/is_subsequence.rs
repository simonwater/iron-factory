//! [392. 判断子序列](https://leetcode.cn/problems/is-subsequence/)
//!
//! 双指针
pub struct Solution;

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let s_len = s.len();
        let t_len = t.len();
        if s_len > t_len {
            return false;
        }
        let s_bytes = s.as_bytes();
        let t_bytes = t.as_bytes();
        let mut i = 0;
        let mut j = 0;
        while i < s_len && j < t_len {
            if s_bytes[i] == t_bytes[j] {
                i += 1;
            }
            j += 1;
        }

        i == s_len
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
