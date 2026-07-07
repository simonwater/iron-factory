/// # [127. 单词接龙](https://leetcode.cn/problems/word-ladder/)
///

pub struct Solution;

use core::str;
use std::collections::HashSet;
use std::collections::VecDeque;
impl Solution {
    //
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let n = word_list.len();
        let mut set: HashSet<String> = HashSet::from_iter(word_list);
        if !set.contains(&end_word) {
            return 0;
        }
        let mut q = VecDeque::with_capacity(n + 1);
        set.remove(&begin_word);
        q.push_back((begin_word, 1));
        while let Some((word, step)) = q.pop_front() {
            let next_words = Self::take_nexts(word, &mut set);
            for next_word in next_words {
                if next_word == end_word {
                    return step + 1;
                }
                q.push_back((next_word, step + 1));
            }
        }
        0
    }

    fn take_nexts(s: String, set: &mut HashSet<String>) -> Vec<String> {
        let mut s_bytes = s.into_bytes();
        let mut ans = Vec::with_capacity(16);
        for i in 0..s_bytes.len() {
            let old = s_bytes[i];
            for c in b'a'..=b'z' {
                if c != old {
                    s_bytes[i] = c;
                    let word = str::from_utf8(&s_bytes).unwrap();
                    if let Some(next) = set.take(word) {
                        ans.push(next);
                    }
                }
            }
            s_bytes[i] = old;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
