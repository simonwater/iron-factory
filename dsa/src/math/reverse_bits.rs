/// # [190. 颠倒二进制位](https://leetcode.cn/problems/reverse-bits/)
///

pub struct Solution;

impl Solution {
    // 二进制循环移位
    pub fn reverse_bits(n: i32) -> i32 {
        let mut n = n;
        let mut ans = 0;
        for i in 0..32 {
            ans |= (n & 1) << (31 - i);
            n = n >> 1;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
