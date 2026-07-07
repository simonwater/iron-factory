/// # [151. 反转字符串中的单词](https://leetcode.cn/problems/reverse-words-in-a-string/)
///

pub struct Solution;

impl Solution {
    //
    pub fn reverse_words(s: String) -> String {
        let bytes = s.as_bytes();
        let mut ans = String::with_capacity(bytes.len());
        let mut left = bytes.len() as i32 - 1;
        while left >= 0 {
            while left >= 0 && bytes[left as usize] == b' ' {
                left -= 1;
            }
            if left < 0 {
                break;
            }
            let right = left;
            while left >= 0 && bytes[left as usize] != b' ' {
                left -= 1;
            }
            ans.push_str(&s[left as usize + 1..right as usize + 1]);
            ans.push_str(" ");
        }
        ans.pop();
        ans
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
