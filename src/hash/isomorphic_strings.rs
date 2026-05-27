use std::collections::HashMap;

/// [205. 同构字符串](https://leetcode.cn/problems/isomorphic-strings/)
struct Solution;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        if s.len() != t.len() || s.is_empty() || t.is_empty() {
            return false;
        }
        let s_bytes = s.as_bytes();
        let t_bytes = t.as_bytes();
        let mut s2t_map = HashMap::with_capacity(128);
        let mut t2s_map = HashMap::with_capacity(128);
        for (&s, &t) in s_bytes.iter().zip(t_bytes.iter()) {
            if *s2t_map.entry(s).or_insert(t) != t {
                return false;
            }
            if *t2s_map.entry(t).or_insert(s) != s {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
