/// [560. 和为 K 的子数组](https://leetcode.cn/problems/subarray-sum-equals-k/)
struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut sum_map = HashMap::with_capacity(nums.len());
        sum_map.insert(0, 1);
        let mut sum = 0;
        let mut ans = 0;
        for num in nums {
            sum += num;
            ans += sum_map.get(&(sum - k)).unwrap_or(&0);
            *sum_map.entry(sum).or_insert(0) += 1;
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
