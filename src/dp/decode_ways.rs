/// # [91. 解码方法](https://leetcode.cn/problems/decode-ways/)
///

struct Solution;

impl Solution {
    //
    pub fn num_decodings(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }
        let bytes = s.as_bytes();
        let mut a = 1;
        let mut b = if bytes[0] == b'0' { 0 } else { 1 };
        for i in 1..s.len() {
            let cur = bytes[i] - b'0';
            let prev = bytes[i - 1] - b'0';
            let mut temp = 0;
            if cur != 0 {
                temp += b;
            }
            let num = prev * 10 + cur;
            if num >= 10 && num <= 26 {
                temp += a;
            }
            if temp == 0 {
                return 0;
            }
            (a, b) = (b, temp);
        }
        b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
