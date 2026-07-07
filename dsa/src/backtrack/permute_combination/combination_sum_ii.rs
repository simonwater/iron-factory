/// [40. 组合总和 II](https://leetcode.cn/problems/combination-sum-ii/)
/// 有重复数字，数字不能重复选择，结果不能有重复项
pub struct Solution;

impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort_unstable();
        let mut ans = Vec::with_capacity(16);
        let mut path = Vec::with_capacity(candidates.len());
        Self::dfs(0, target, &candidates, &mut path, &mut ans);
        ans
    }

    // 对每一个候选数字都做出选/不选两种决策
    fn dfs(mut i: usize, target: i32, nums: &[i32], path: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
        if target == 0 {
            ans.push(path.clone());
            return;
        }
        if i == nums.len() || nums[i] > target {
            return;
        }
        path.push(nums[i]);
        Self::dfs(i + 1, target - nums[i], nums, path, ans);
        path.pop();

        while i < nums.len() - 1 && nums[i] == nums[i + 1] {
            i += 1;
        }
        Self::dfs(i + 1, target, nums, path, ans);
    }
}

pub struct Solution2;

impl Solution2 {
    // 对答案中的每一个位置，都枚举所有可能填入的数字
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort_unstable();
        let mut ans = Vec::with_capacity(16);
        let mut path = Vec::with_capacity(candidates.len());
        Self::dfs(0, target, &candidates, &mut path, &mut ans);
        ans
    }

    // 枚举所有可以填入path当前层（末尾）的数字，根据组合的性质，(1,2)和(2,1)相同
    // 所以当前位置选定以后，下一个数字只能往前选，不能回头选已经选过的
    fn dfs(start: usize, target: i32, nums: &[i32], path: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
        if target == 0 {
            ans.push(path.clone());
            return;
        }
        if start == nums.len() || nums[start] > target {
            return;
        }
        for i in start..nums.len() {
            // 对于枚举到的这个数字，path的当前层在前面的迭代中已经用过相同数，再继续用就会出现重复答案
            if i > start && nums[i] == nums[i - 1] {
                continue;
            }
            path.push(nums[i]);
            Self::dfs(i + 1, target - nums[i], nums, path, ans);
            path.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
