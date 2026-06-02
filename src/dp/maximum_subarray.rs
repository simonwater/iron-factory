/// [53. 最大子数组和](https://leetcode.cn/problems/maximum-subarray/)
struct Solution;
/// 动态规划
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut ans = nums[0];
        let mut cur = nums[0];
        for &num in nums.iter().skip(1) {
            cur = num.max(cur + num);
            ans = ans.max(cur);
        }
        ans
    }
}

/// 前缀和。理论上性能不如动态规划，因为循环内部逻辑间有数据指令的依赖关系，无法利用CPU的并行预取
struct Solution2;
impl Solution2 {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut ans = nums[0];
        let mut prev_min = 0;
        let mut cur_sum = 0;
        for &num in &nums {
            cur_sum += num;
            ans = ans.max(cur_sum - prev_min);
            prev_min = prev_min.min(cur_sum);
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
