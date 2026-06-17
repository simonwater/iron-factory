/// # [152. 乘积最大子数组](https://leetcode.cn/problems/maximum-product-subarray/)
///

struct Solution;

impl Solution {
    //
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut pre_max = 1;
        let mut pre_min = 1;
        let mut ans = nums[0];
        for num in nums {
            let max = pre_max;
            let min = pre_min;
            pre_max = num.max((num * max).max(num * min));
            pre_min = num.min((num * max).min(num * min));
            ans = ans.max(pre_max);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
