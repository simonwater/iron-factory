//! [30. 串联所有单词的子串](https://leetcode.cn/problems/substring-with-concatenation-of-all-words/)
//!
//! 定长滑动窗口，窗口以单词长度为步长向右滑动。为了枚举所有可能，滑动窗口判断word_len此

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let words_len = words.len();
        let mut words_map: HashMap<&[u8], i32> = HashMap::with_capacity(words_len);
        for s in words.iter() {
            *words_map.entry(s.as_bytes()).or_insert(0) += 1;
        }
        let s_bytes = s.as_bytes();
        let mut ans = Vec::with_capacity(32);
        let word_len = words[0].len();
        let win_len = words_len * word_len;
        if win_len > s.len() {
            return ans;
        }
        for start in 0..word_len {
            let mut cur_map: HashMap<&[u8], i32> = HashMap::with_capacity(words_len);
            let mut left = start;
            let mut right = start + word_len;
            while right <= s.len() {
                let in_word = &s_bytes[right - word_len..right];
                if !words_map.contains_key(in_word) {
                    left = right;
                    right += word_len;
                    cur_map.clear();
                    continue;
                }
                *cur_map.entry(in_word).or_insert(0) += 1;

                if right - left == win_len {
                    if cur_map == words_map {
                        ans.push(left as i32);
                    }
                    let out_word = &s_bytes[left..left + word_len];
                    let val_p = cur_map.get_mut(out_word).unwrap();
                    *val_p -= 1;
                    if *val_p == 0 {
                        cur_map.remove(out_word);
                    }
                    left += word_len;
                }
                right += word_len;
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
