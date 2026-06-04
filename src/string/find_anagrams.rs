/// [438. 找到字符串中所有字母异位词](https://leetcode.cn/problems/find-all-anagrams-in-a-string/)
struct Solution;

impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        if s.is_empty() || p.is_empty() || s.len() < p.len() {
            return vec![];
        }
        let s_bytes = s.as_bytes();
        let p_bytes = p.as_bytes();
        let mut s_cnt = [0; 26];
        let mut p_cnt = [0; 26];
        for &c in p_bytes {
            let idx = (c - b'a') as usize;
            p_cnt[idx] += 1;
        }

        let mut ans = Vec::with_capacity(s.len() - p.len() + 1);
        for (i, &c) in s_bytes.iter().enumerate() {
            let cur_char_idx = (c - b'a') as usize;
            s_cnt[cur_char_idx] += 1;
            if i < p.len() - 1 {
                continue;
            }

            let left = i + 1 - p.len();
            if s_cnt == p_cnt {
                ans.push(left as i32);
            }
            let left_char_idx = (s_bytes[left] - b'a') as usize;
            s_cnt[left_char_idx] -= 1;
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
