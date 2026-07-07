/// # [198. 打家劫舍](https://leetcode.cn/problems/house-robber/)
///

pub struct Solution;

impl Solution {
    //
    pub fn rob(nums: Vec<i32>) -> i32 {
        Self::rob_inner(&nums)
    }

    pub fn rob2(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }
        Self::rob_inner(&nums[1..]).max(Self::rob_inner(&nums[..(nums.len() - 1)]))
    }

    pub fn rob_inner(nums: &[i32]) -> i32 {
        let (mut a, mut b) = (0, nums[0]);
        for &num in nums.iter().skip(1) {
            (a, b) = (b, (num + a).max(b));
        }
        b
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
