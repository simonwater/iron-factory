/// # [201. 数字范围按位与](https://leetcode.cn/problems/bitwise-and-of-numbers-range/)
/// 等价于找两个二进制序列的最长公共前缀

pub struct Solution;

impl Solution {
    pub fn range_bitwise_and(mut left: i32, mut right: i32) -> i32 {
        let mut cnt = 0;
        while left != right {
            left = left >> 1;
            right = right >> 1;
            cnt += 1;
        }
        left << cnt
    }
}

pub struct Solution2;

impl Solution2 {
    pub fn range_bitwise_and(left: i32, mut right: i32) -> i32 {
        while left < right {
            right = right & (right - 1); // 将 right 最右侧的一个 1 抹平为 0
        }
        right
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
