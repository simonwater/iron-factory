/// [76. 最小覆盖子串](https://leetcode.cn/problems/minimum-window-substring/description/)
pub struct Solution;

impl Solution {
    // 对比两个哈希中的每一项来判断是否覆盖
    pub fn min_window(s: String, t: String) -> String {
        if s.len() < t.len() {
            return String::new();
        }
        let s = s.as_bytes();
        let t = t.as_bytes();
        let mut s_cnt = [0; 60];
        let mut t_cnt = [0; 60];
        for &c in t {
            let idx = (c - b'A') as usize;
            t_cnt[idx] += 1;
        }

        let mut start = 0;
        let mut end = 0;
        let mut ans_len = usize::MAX;
        let mut left = 0;
        for (right, &c) in s.iter().enumerate() {
            let cur_c_idx = (c - b'A') as usize;
            s_cnt[cur_c_idx] += 1;
            if right - left + 1 < t.len() {
                continue;
            }

            while Self::contains(&s_cnt, &t_cnt) {
                let win_len = right - left + 1;
                if win_len == t.len() {
                    return std::str::from_utf8(&s[left..right + 1])
                        .unwrap()
                        .to_string();
                }
                if win_len < ans_len {
                    ans_len = win_len;
                    start = left;
                    end = right;
                }

                let left_c_idx = (s[left] - b'A') as usize;
                s_cnt[left_c_idx] -= 1;
                left += 1;
            }
        }

        if ans_len == usize::MAX {
            String::new()
        } else {
            std::str::from_utf8(&s[start..end + 1]).unwrap().to_string()
        }
    }

    fn contains(s: &[i32; 60], t: &[i32; 60]) -> bool {
        s.iter().zip(t.iter()).all(|(&a, &b)| a >= b)
    }

    // 维护独立字符量来判断是否覆盖
    pub fn min_window2(s: String, t: String) -> String {
        if s.len() < t.len() {
            return String::new();
        }
        let s = s.as_bytes();
        let t = t.as_bytes();
        let mut s_cnt = [0; 60];
        let mut t_cnt = [0; 60];
        let mut required = 0; // 需要的独立字符量
        for &c in t {
            let idx = (c - b'A') as usize;
            if t_cnt[idx] == 0 {
                required += 1;
            }
            t_cnt[idx] += 1;
        }

        let mut start = 0;
        let mut end = 0;
        let mut ans_len = usize::MAX;
        let mut left = 0;
        let mut formed = 0; // 构成的独立字符量
        for (right, &c) in s.iter().enumerate() {
            let cur_c_idx = (c - b'A') as usize;
            s_cnt[cur_c_idx] += 1;
            if s_cnt[cur_c_idx] == t_cnt[cur_c_idx] {
                formed += 1;
            }
            if right - left + 1 < t.len() {
                continue;
            }

            while formed == required {
                let win_len = right - left + 1;
                if win_len == t.len() {
                    return std::str::from_utf8(&s[left..right + 1])
                        .unwrap()
                        .to_string();
                }
                if win_len < ans_len {
                    ans_len = win_len;
                    start = left;
                    end = right;
                }

                let left_c_idx = (s[left] - b'A') as usize;
                s_cnt[left_c_idx] -= 1;
                if s_cnt[left_c_idx] < t_cnt[left_c_idx] {
                    formed -= 1;
                }
                left += 1;
            }
        }

        if ans_len == usize::MAX {
            String::new()
        } else {
            std::str::from_utf8(&s[start..end + 1]).unwrap().to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let s = String::from("ADOBECODEBANC");
        let t = String::from("ABC");
        let ans = String::from("BANC");
        assert_eq!(Solution::min_window(s, t), ans);

        let s = String::from("a");
        let t = String::from("a");
        let ans = String::from("a");
        assert_eq!(Solution::min_window(s, t), ans);

        let s = String::from("a");
        let t = String::from("aa");
        let ans = String::from("");
        assert_eq!(Solution::min_window(s, t), ans);

        let s = String::from("a");
        let t = String::from("b");
        let ans = String::from("");
        assert_eq!(Solution::min_window(s, t), ans);
    }

    #[test]
    fn test2() {
        let s = String::from("ADOBECODEBANC");
        let t = String::from("ABC");
        let ans = String::from("BANC");
        assert_eq!(Solution::min_window2(s, t), ans);

        let s = String::from("a");
        let t = String::from("a");
        let ans = String::from("a");
        assert_eq!(Solution::min_window2(s, t), ans);

        let s = String::from("a");
        let t = String::from("aa");
        let ans = String::from("");
        assert_eq!(Solution::min_window2(s, t), ans);

        let s = String::from("a");
        let t = String::from("b");
        let ans = String::from("");
        assert_eq!(Solution::min_window2(s, t), ans);
    }
}
