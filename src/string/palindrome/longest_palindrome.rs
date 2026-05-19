/// [409. 最长回文串](https://leetcode.cn/problems/longest-palindrome/description/)
struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut cnts = [0; 60];
        let s = s.as_bytes();
        for &c in s {
            let idx = (c - b'A') as usize;
            cnts[idx] += 1;
        }
        let mut odd_cnt = 0;
        for cnt in cnts {
            if cnt % 2 == 1 {
                odd_cnt += 1;
            }
        }
        let len = s.len() as i32;
        if odd_cnt > 0 { len - odd_cnt + 1 } else { len }
    }

    /// [5. 最长回文子串](https://leetcode.cn/problems/longest-palindromic-substring)
    pub fn longest_palindrome_substring(s: String) -> String {
        if s.len() <= 1 {
            return s;
        }
        let bytes = s.as_bytes();
        let mut start = 0;
        let mut end = 0;
        let mut len = 1;
        for i in 0..s.len() - 1 {
            let (s, e, l) = Self::check_palindrome(bytes, i, i);
            if l > len {
                (start, end, len) = (s, e, l);
            }
            let (s, e, l) = Self::check_palindrome(bytes, i, i + 1);
            if l > len {
                (start, end, len) = (s, e, l);
            }
        }

        std::str::from_utf8(&bytes[start..end + 1])
            .unwrap()
            .to_string()
    }

    fn check_palindrome(s: &[u8], mut left: usize, mut right: usize) -> (usize, usize, usize) {
        while right < s.len() {
            if s[left] != s[right] {
                break;
            }
            if left == 0 || right == s.len() - 1 {
                return (left, right, right - left + 1);
            }
            right += 1;
            left -= 1;
        }
        (left + 1, right - 1, right - left - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let s = String::from("abccccdd");
        let ans = 7;
        assert_eq!(Solution::longest_palindrome(s), ans);

        let s = String::from("a");
        let ans = 1;
        assert_eq!(Solution::longest_palindrome(s), ans);

        let s = String::from("aa");
        let ans = 2;
        assert_eq!(Solution::longest_palindrome(s), ans);
    }

    #[test]
    fn test2() {
        let s = String::from("babad");
        let ans = String::from("bab");
        assert_eq!(Solution::longest_palindrome_substring(s), ans);

        let s = String::from("cbbd");
        let ans = String::from("bb");
        assert_eq!(Solution::longest_palindrome_substring(s), ans);

        let s = String::from("aaaa");
        let ans = String::from("aaaa");
        assert_eq!(Solution::longest_palindrome_substring(s), ans);
    }
}
