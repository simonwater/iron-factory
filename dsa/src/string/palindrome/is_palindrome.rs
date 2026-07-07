/// [125. 验证回文串](https://leetcode.cn/problems/valid-palindrome/description/)
pub struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let bytes = s.as_bytes();
        let mut i = 0;
        let mut j = bytes.len() - 1;
        while i < j {
            while i < j && !bytes[i].is_ascii_alphanumeric() {
                i += 1;
            }
            while i < j && !bytes[j].is_ascii_alphanumeric() {
                j -= 1;
            }
            if i == j {
                break;
            }
            if bytes[i].to_ascii_lowercase() != bytes[j].to_ascii_lowercase() {
                return false;
            }
            i += 1;
            j -= 1; // 谨防溢出
        }
        true
    }

    pub fn is_palindrome_iter(s: String) -> bool {
        // 1. 建立一个纯净的字节流：过滤掉非字母数字 -> 统一转为 ASCII 小写
        let clean_bytes = s
            .as_bytes()
            .iter()
            .filter(|&&b| b.is_ascii_alphanumeric())
            .map(|&b| b.to_ascii_lowercase());

        // 2. 克隆的仅仅是迭代器指针本身（几个字节的栈轻量结构），而不是底层的字符串数据，开销为 0
        clean_bytes.clone().eq(clean_bytes.rev())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let s = String::from("A man, a plan, a canal: Panama");
        assert_eq!(Solution::is_palindrome(s), true);

        let s = String::from("race a car");
        assert_eq!(Solution::is_palindrome(s), false);

        let s = String::from(" ");
        assert_eq!(Solution::is_palindrome(s), true);

        let s = String::from("a.");
        assert_eq!(Solution::is_palindrome(s), true);
    }

    #[test]
    fn test2() {
        let s = String::from("A man, a plan, a canal: Panama");
        assert_eq!(Solution::is_palindrome_iter(s), true);

        let s = String::from("race a car");
        assert_eq!(Solution::is_palindrome_iter(s), false);

        let s = String::from(" ");
        assert_eq!(Solution::is_palindrome_iter(s), true);

        let s = String::from("a.");
        assert_eq!(Solution::is_palindrome_iter(s), true);
    }
}
