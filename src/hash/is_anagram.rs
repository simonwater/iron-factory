/// [242. 有效的字母异位词](https://leetcode.cn/problems/valid-anagram/description/)
struct Solution;

impl Solution {
    /// 使用栈上固定大小的数组记录字符频次，相比堆上哈希表性能高很多。
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut cnt = [0; 26]; // 直接在栈上开辟定长数组，相比堆上的动态数组性能更高
        let s = s.as_bytes();
        let t = t.as_bytes();
        // 使用解构直接把值剥离出来，避免频繁的解引用
        for &c in s {
            let i = (c - b'a') as usize;
            cnt[i] += 1;
        }
        for &c in t {
            let i = (c - b'a') as usize;
            cnt[i] -= 1;
        }

        cnt.iter().all(|&a| a == 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let s = String::from("anagram");
        let t = String::from("nagaram");
        assert_eq!(Solution::is_anagram(s, t), true);

        let s = String::from("rat");
        let t = String::from("car");
        assert_eq!(Solution::is_anagram(s, t), false);
    }
}
