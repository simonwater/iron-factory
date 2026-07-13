//! [28. 找出字符串中第一个匹配项的下标](https://leetcode.cn/problems/find-the-index-of-the-first-occurrence-in-a-string/)
//!

pub struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if haystack.len() < needle.len() {
            return -1;
        }
        let bytes1 = haystack.as_bytes();
        let bytes2 = needle.as_bytes();
        for i in 0..=bytes1.len() - bytes2.len() {
            let mut j = i;
            for k in 0..bytes2.len() {
                if bytes1[j] != bytes2[k] {
                    break;
                }
                j += 1;
            }
            if j == i + bytes2.len() {
                return i as i32;
            }
        }
        -1
    }
}

pub struct Solution2;

impl Solution2 {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let n1 = haystack.len();
        let n2 = needle.len();
        if n1 < n2 {
            return -1;
        }
        for i in 0..=n1 - n2 {
            if &haystack[i..i + n2] == needle {
                return i as i32;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
