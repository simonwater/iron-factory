/// # [567. 字符串的排列](https://leetcode.cn/problems/permutation-in-string/)
///

pub struct Solution;

impl Solution {
    //
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let mut s1_cnt = [0; 26];
        for &c in s1.as_bytes() {
            s1_cnt[(c - b'a') as usize] += 1;
        }
        let mut win = [0; 26];
        let s2_bytes = s2.as_bytes();
        for (i, &c) in s2_bytes.iter().enumerate() {
            win[(c - b'a') as usize] += 1;
            if i < s1.len() - 1 {
                continue;
            }
            if win == s1_cnt {
                return true;
            }
            let left_c = s2_bytes[i + 1 - s1.len()];
            win[(left_c - b'a') as usize] -= 1;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
