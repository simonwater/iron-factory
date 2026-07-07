/// [680. 验证回文串 II](https://leetcode.cn/problems/valid-palindrome-ii/description/)
pub struct Solution;

impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        if s.is_empty() {
            return true;
        }
        let mut i = 0;
        let mut j = s.len() - 1;
        let s = s.as_bytes();
        while i < j {
            if s[i] != s[j] {
                return Self::is_palindrome(&s[i + 1..j + 1]) || Self::is_palindrome(&s[i..j]);
            }
            i += 1;
            j -= 1;
        }
        true
    }

    fn is_palindrome(s: &[u8]) -> bool {
        if s.is_empty() {
            return true;
        }
        let mut i = 0;
        let mut j = s.len() - 1;
        while i < j {
            if s[i] != s[j] {
                return false;
            }
            i += 1;
            j -= 1;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let s = String::from("aba");
        assert_eq!(Solution::valid_palindrome(s), true);

        let s = String::from("abca");
        assert_eq!(Solution::valid_palindrome(s), true);

        let s = String::from("abc");
        assert_eq!(Solution::valid_palindrome(s), false);
    }
}
