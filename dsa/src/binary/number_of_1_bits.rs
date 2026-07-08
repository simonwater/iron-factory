/// [191. 位1的个数](https://leetcode.cn/problems/number-of-1-bits/description/)
pub struct Solution;

impl Solution {
    pub fn hamming_weight(mut n: i32) -> i32 {
        let mut ans = 0;
        while n > 0 {
            ans += n & 1;
            n = n >> 1;
        }
        ans
    }
}

pub struct Solution2;

impl Solution2 {
    pub fn hamming_weight(mut n: i32) -> i32 {
        let mut ans = 0;
        while n > 0 {
            n &= n - 1; // 将 n 最右侧的一个 1 抹平为 0
            ans += 1;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::hamming_weight(11), 3);
        assert_eq!(Solution::hamming_weight(128), 1);
        assert_eq!(Solution::hamming_weight(2147483645), 30);
    }
}
