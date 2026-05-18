/// [3. 无重复字符的最长子串](https://leetcode.cn/problems/longest-substring-without-repeating-characters)
struct Solution;

use std::collections::HashSet;
impl Solution {
    // 通过哈希表记录是否已经存在，如果存在，需要从左边界开始一个一个的迭代删除。
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.len() <= 1 {
            return s.len() as i32;
        }
        let mut set = HashSet::new();
        let bytes = s.as_bytes();
        let mut ans = 1;
        for (i, &c) in bytes.iter().enumerate() {
            while set.contains(&c) {
                set.remove(&bytes[i - set.len()]);
            }
            set.insert(c);
            ans = ans.max(set.len());
        }
        ans as i32
    }

    // 记录下每个字符最近出现的位置，如果新字符位置在窗口内，窗口的左边界直接跳到最近一次位置的后面。
    // 此处相比方法一一个位置一个位置移动性能更高
    pub fn length_of_longest_substring2(s: String) -> i32 {
        if s.len() <= 1 {
            return s.len() as i32;
        }
        let mut last_pos = [-1; 128];
        let mut left = 0;
        let mut ans = 1;
        for (right, &c) in s.as_bytes().iter().enumerate() {
            let i = c as usize;
            if last_pos[i] >= left {
                left = last_pos[i] + 1;
            }
            last_pos[i] = right as i32;
            let len = right as i32 - left + 1;
            if len > ans {
                ans = len;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let s = String::from("abcabcbb");
        let ans = 3;
        assert_eq!(Solution::length_of_longest_substring(s), ans);

        let s = String::from("bbbbb");
        let ans = 1;
        assert_eq!(Solution::length_of_longest_substring(s), ans);

        let s = String::from("pwwkew");
        let ans = 3;
        assert_eq!(Solution::length_of_longest_substring(s), ans);
    }

    #[test]
    fn test2() {
        let s = String::from("abcabcbb");
        let ans = 3;
        assert_eq!(Solution::length_of_longest_substring2(s), ans);

        let s = String::from("bbbbb");
        let ans = 1;
        assert_eq!(Solution::length_of_longest_substring2(s), ans);

        let s = String::from("pwwkew");
        let ans = 3;
        assert_eq!(Solution::length_of_longest_substring2(s), ans);
    }
}
