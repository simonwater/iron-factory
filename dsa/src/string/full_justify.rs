//! [68. 文本左右对齐](https://leetcode.cn/problems/text-justification/)
//!

pub struct Solution;

impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let max_width = max_width as usize;
        let mut ans = Vec::with_capacity(32);
        let mut words_len = 0;
        let mut left = 0;
        for (i, word) in words.iter().enumerate() {
            let blank_cnt = i - left; // 需要填充的最少空格数
            if words_len + blank_cnt + word.len() > max_width {
                // 加上当前 word 就会溢出, 收集一行记录
                // 注意切片右边界：不包含当前单词
                ans.push(Self::make_line(&words[left..i], words_len, max_width));

                words_len = word.len();
                left = i;
            } else {
                words_len += word.len();
            }
        }

        // 最后一行
        ans.push(Self::make_last_line(&words[left..], words_len, max_width));

        ans
    }

    fn make_line(buffer: &[String], words_len: usize, max_width: usize) -> String {
        let mut ans = String::with_capacity(max_width);
        let total_blanks = max_width - words_len; // 需要填充的空格总数
        let slots = buffer.len() - 1;
        if slots == 0 {
            ans.push_str(&buffer[0]);
            Self::fill_blanks(&mut ans, total_blanks);
        } else {
            let base_blanks = total_blanks / slots;
            let mut wider_slots = total_blanks % slots;
            for str in buffer {
                if !ans.is_empty() {
                    Self::fill_blanks(&mut ans, base_blanks);
                    if wider_slots > 0 {
                        ans.push(' ');
                        wider_slots -= 1;
                    }
                }
                ans.push_str(str);
            }
        }
        ans
    }

    /// 最后一行特殊处理
    fn make_last_line(buffer: &[String], words_len: usize, max_width: usize) -> String {
        let mut ans = String::with_capacity(max_width);
        let mut total_blanks = max_width - words_len; // 需要填充的空格总数
        for str in buffer {
            if !ans.is_empty() {
                ans.push(' ');
                total_blanks -= 1;
            }
            ans.push_str(str);
        }
        Self::fill_blanks(&mut ans, total_blanks);
        ans
    }

    fn fill_blanks(buffer: &mut String, cnt: usize) {
        for _ in 0..cnt {
            buffer.push(' ');
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let words: Vec<String> = [
            "This",
            "is",
            "an",
            "example",
            "of",
            "text",
            "justification.",
        ]
        .iter()
        .map(|&s| String::from(s))
        .collect();
        Solution::full_justify(words, 16);
    }
}
