/// [47. 全排列 II](https://leetcode.cn/problems/permutations-ii/)
struct Solution;
use std::collections::HashSet;
impl Solution {
    pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = Vec::with_capacity(16);
        Self::dfs(0, &mut nums, &mut ans);
        ans
    }

    fn dfs(start: usize, nums: &mut [i32], ans: &mut Vec<Vec<i32>>) {
        if start == nums.len() {
            ans.push(nums.to_vec());
            return;
        }
        let mut used_nums = HashSet::with_capacity(nums.len() - start);
        for i in start..nums.len() {
            let cur = nums[i];
            if !used_nums.insert(cur) {
                continue;
            }
            nums.swap(start, i);
            Self::dfs(start + 1, nums, ans);
            nums.swap(start, i);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
