/// # [268. 丢失的数字](https://leetcode.cn/problems/missing-number/)
///

pub struct Solution;
/// nums = [9,6,4,2,3,5,7,0,1] ans = 8
impl Solution {
    // 求和
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let sum: i32 = nums.iter().sum();
        n * (n - 1) / 2 - sum
    }
}

/// 异或
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
