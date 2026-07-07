/// [692. 前K个高频单词](https://leetcode.cn/problems/top-k-frequent-words/)
pub struct Solution;

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Eq, PartialEq)]
struct WordItem {
    cnt: i32,
    word: String,
}

impl Ord for WordItem {
    fn cmp(&self, other: &Self) -> Ordering {
        let ord = other.cnt.cmp(&self.cnt); // 对cnt只进行一次比较
        if ord != Ordering::Equal {
            return ord;
        }
        self.word.cmp(&other.word)
    }
}

impl PartialOrd for WordItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
        let mut counter = HashMap::new();
        for word in words {
            *counter.entry(word).or_insert(0) += 1;
        }

        let k = k as usize;
        let mut heap = BinaryHeap::with_capacity(k);
        for (word, cnt) in counter {
            let item = WordItem { word, cnt };
            if heap.len() < k {
                heap.push(item);
            } else {
                if let Some(mut top) = heap.peek_mut() {
                    if item < *top {
                        *top = item;
                    }
                }
            }
        }
        let mut ans = Vec::with_capacity(k);
        while let Some(item) = heap.pop() {
            ans.push(item.word);
        }
        ans.reverse();
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let words = ["i", "love", "leetcode", "i", "love", "coding"];
        let k = 2;
        let ans = ["i", "love"];
        let words: Vec<String> = words.into_iter().map(String::from).collect();
        let ans: Vec<String> = ans.into_iter().map(String::from).collect();
        assert_eq!(Solution::top_k_frequent(words, k), ans);

        let words = [
            "the", "day", "is", "sunny", "the", "the", "the", "sunny", "is", "is",
        ];
        let k = 4;
        let ans = ["the", "is", "sunny", "day"];
        let words: Vec<String> = words.into_iter().map(String::from).collect();
        let ans: Vec<String> = ans.into_iter().map(String::from).collect();
        assert_eq!(Solution::top_k_frequent(words, k), ans);
    }
}
