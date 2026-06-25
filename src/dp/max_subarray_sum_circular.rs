/// # [918. 环形子数组的最大和](https://leetcode.cn/problems/maximum-sum-circular-subarray/)
/// 最大子数组和与最小子数组和的结合，需要特殊判断全负的情况
///

struct Solution;

impl Solution {
    //
    pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
        let mut max_sub = nums[0];
        let mut pre_max = 0;

        let mut min_sub = nums[0];
        let mut pre_min = 0;
        let mut sum = 0;
        for num in nums {
            sum += num;
            let cur_max = num.max(num + pre_max);
            max_sub = max_sub.max(cur_max);
            pre_max = cur_max;

            let cur_min = num.min(num + pre_min);
            min_sub = min_sub.min(cur_min);
            pre_min = cur_min;
        }
        if max_sub < 0 {
            max_sub
        } else {
            max_sub.max(sum - min_sub)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![-10, -7, 9, -7, 6, 9, -9, -4, -8, -5];
        assert_eq!(17, Solution::max_subarray_sum_circular(nums));

        let nums = vec![-3, -2, -3];
        assert_eq!(-2, Solution::max_subarray_sum_circular(nums));
    }
}
