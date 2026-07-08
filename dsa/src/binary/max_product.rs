/// # [318. 最大单词长度乘积](https://leetcode.cn/problems/maximum-product-of-word-lengths/)
/// 基本思路：不含有公共字母的单词对 => 两个二进制序列a和b，相同位置不会同时为1， 则 a & b = 0
/// 进一步优化：对于bitmask相同的字符串，只需保留长度最长的即可

pub struct Solution;

impl Solution {
    pub fn max_product(words: Vec<String>) -> i32 {
        let n = words.len();
        let mut masks = vec![0; n];
        for (i, w) in words.iter().enumerate() {
            let mut mask = 0;
            for &c in w.as_bytes() {
                let ch_idx = (c - b'a') as usize; // [0, 25];
                mask |= 1 << ch_idx;
            }
            masks[i] = mask;
        }
        let mut ans = 0;
        for i in 1..words.len() {
            for j in 0..i {
                if masks[i] & masks[j] == 0 {
                    ans = ans.max(words[i].len() * words[j].len());
                }
            }
        }
        ans as i32
    }
}

/// 关注哈希map的语义化接口and_modify / or_insert
pub struct Solution2;
use std::collections::HashMap;
impl Solution2 {
    pub fn max_product(words: Vec<String>) -> i32 {
        let n = words.len();
        let mut map = HashMap::with_capacity(n);
        for (i, w) in words.iter().enumerate() {
            let mut mask = 0;
            let len = words[i].len();
            for &c in w.as_bytes() {
                let ch_idx = (c - b'a') as usize; // [0, 25];
                mask |= 1 << ch_idx;
            }
            map.entry(mask)
                .and_modify(|e| *e = len.max(*e))
                .or_insert(len);
        }
        let masks = map.into_iter().collect::<Vec<(i32, usize)>>();
        let mut ans = 0;
        for i in 1..masks.len() {
            let (mask1, len1) = masks[i];
            for j in 0..i {
                let (mask2, len2) = masks[j];
                if mask1 & mask2 == 0 {
                    ans = ans.max(len1 * len2);
                }
            }
        }
        ans as i32
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
