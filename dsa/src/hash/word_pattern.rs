/// [290. 单词规律](https://leetcode.cn/problems/word-pattern/)
pub struct Solution;

use std::collections::HashMap;
impl Solution {
    /// 哈希表：
    /// 1. 字符用u8不仅消灭了 UTF-8 字符解码的 CPU 开销，还让哈希表的 Key 变成了最轻量的类型
    /// 2. 使用字符串切片没有对输入的 s 进行二次堆内存切片和复制。所有的单词全是通过指针直接悬挂在 s 上
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let p_bytes = pattern.as_bytes();
        let mut words = s.split_whitespace();
        let mut pat_2_word: HashMap<u8, &str> = HashMap::new();
        let mut word_2_pat: HashMap<&str, u8> = HashMap::new();
        for &pat in p_bytes {
            if let Some(word) = words.next() {
                if *pat_2_word.entry(pat).or_insert(word) != word {
                    return false;
                }
                if *word_2_pat.entry(word).or_insert(pat) != pat {
                    return false;
                }
            } else {
                return false;
            }
        }
        words.next().is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let pattern = String::from("abba");
        let s = String::from("dog cat cat dog");
        let ans = true;
        assert_eq!(Solution::word_pattern(pattern, s), ans);

        let pattern = String::from("abba");
        let s = String::from("dog cat cat fish");
        let ans = false;
        assert_eq!(Solution::word_pattern(pattern, s), ans);

        let pattern = String::from("aaaa");
        let s = String::from("dog cat cat dog");
        let ans = false;
        assert_eq!(Solution::word_pattern(pattern, s), ans);

        let pattern = String::from("abba");
        let s = String::from("dog dog dog dog");
        let ans = false;
        assert_eq!(Solution::word_pattern(pattern, s), ans);
    }
}
