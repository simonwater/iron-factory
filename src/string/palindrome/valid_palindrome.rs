/// [680. 验证回文串 II](https://leetcode.cn/problems/valid-palindrome-ii/description/)
struct Solution;

impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
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
