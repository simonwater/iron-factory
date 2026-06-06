/// [394. 字符串解码](https://leetcode.cn/problems/decode-string/)
struct Solution;

impl Solution {
    pub fn decode_string(s: String) -> String {
        let mut num_stack = Vec::with_capacity(16);
        let mut str_stack = Vec::with_capacity(16);
        let mut cur_str = String::with_capacity(16);
        let mut num = 0;
        for &c in s.as_bytes() {
            match c {
                b'0'..=b'9' => num = (c - b'0') as usize + num * 10,
                b'[' => {
                    num_stack.push(num);
                    str_stack.push(cur_str);
                    cur_str = String::with_capacity(16);
                    num = 0;
                }
                b']' => {
                    let n = num_stack.pop().unwrap();
                    let mut prev = str_stack.pop().unwrap();
                    // 字符串可以原地扩容，相比定义新缓冲区能减少一次拷贝
                    prev.reserve(n * cur_str.len());
                    for _ in 0..n {
                        prev.push_str(&cur_str);
                    }
                    cur_str = prev;
                }
                _ => cur_str.push(c as char),
            }
        }
        cur_str
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
