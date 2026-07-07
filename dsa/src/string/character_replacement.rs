/// [424. 替换后的最长重复字符](https://leetcode.cn/problems/longest-repeating-character-replacement/)
pub struct Solution;

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let n = s.len() as i32;
        if k >= n - 1 {
            return n as i32;
        }
        let s_bytes = s.as_bytes();
        let mut cnt = [0i32; 26];
        let mut ans = k + 1;
        let mut left = 0usize;
        let k_usize = k as usize;
        for (right, &c) in s_bytes.iter().enumerate() {
            cnt[(c - b'A') as usize] += 1;
            if right <= k_usize {
                continue;
            }

            if !Self::check(&cnt, k, (right - left) as i32 + 1) {
                // 左边界只需收缩一位，没必要循环得到一个可行解，因为当前右边界已经固定，只收缩左边界得到的解肯定不可能更好。
                cnt[(s_bytes[left] - b'A') as usize] -= 1;
                left += 1;
            }
            ans = ans.max((right - left) as i32 + 1);
        }

        ans
    }

    fn check(cnt: &[i32; 26], k: i32, total: i32) -> bool {
        if k >= total - 1 {
            return true;
        }
        let max = cnt.iter().max().unwrap();
        return k >= total - max;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
