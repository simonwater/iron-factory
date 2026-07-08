/// # [268. 丢失的数字](https://leetcode.cn/problems/missing-number/)
/// 把[0, n]的 n + 1 个数字放入长度为n的数组。

pub struct Solution;
/// nums = [9,6,4,2,3,5,7,0,1] ans = 8
impl Solution {
    // 求和再相减
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let sum: i32 = nums.iter().sum();
        n * (n - 1) / 2 - sum
    }
}

/// 位运算异或的性质是相同两数异或得0，数字和下标异或，最后等价于： [136. 只出现一次的数字]
pub struct Solution2;
/// nums = [9,6,4,2,3,5,7,0,1] ans = 8
impl Solution2 {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut ans = nums.len() as i32;
        for (i, &num) in nums.iter().enumerate() {
            ans = ans ^ num ^ (i as i32);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
