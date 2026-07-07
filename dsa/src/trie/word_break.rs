/// [139. 单词拆分](https://leetcode.cn/problems/word-break/)
pub struct Solution;
use std::collections::HashSet;
impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let set: HashSet<String> = HashSet::from_iter(word_dict);
        let n = s.len();
        let mut dp = vec![false; n + 1]; // dp[n] 表示前n个字符(s[0..=n-1])是否可以通过dict拼接而成
        dp[0] = true; // 为后续计算方便，空字符串约定为可以拼接而成。
        for i in 1..=n {
            for j in 0..i {
                if dp[j] && set.contains(&s[j..i]) {
                    dp[i] = true;
                    break;
                }
            }
        }

        dp[n]
    }
}

pub struct Solution2;
use crate::trie::Trie;
impl Solution2 {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let mut trie = Trie::new();
        for word in word_dict {
            trie.insert(word);
        }
        let n = s.len();
        let mut dp = vec![false; n + 1]; // dp[i] 表示前i个字符(s[0..=i-1])是否可以通过dict拼接而成
        dp[0] = true; // 为后续计算方便，空字符串约定为可以拼接而成。
        for i in 1..=n {
            for j in 0..i {
                if dp[j] && trie.search_str(&s[j..i]) {
                    dp[i] = true;
                    break;
                }
            }
        }

        dp[n]
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
