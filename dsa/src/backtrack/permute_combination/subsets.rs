/// [78. 子集](https://leetcode.cn/problems/subsets/)
pub struct Solution;

// 对每个数字都做选/不选两种决策，然后递归处理下一个数
impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = Vec::with_capacity(16);
        let mut path = Vec::with_capacity(nums.len());
        Self::dfs(0, &nums, &mut path, &mut ans);
        ans
    }

    fn dfs(i: usize, nums: &[i32], path: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
        if i == nums.len() {
            ans.push(path.clone());
            return;
        }
        Self::dfs(i + 1, nums, path, ans);
        path.push(nums[i]);
        Self::dfs(i + 1, nums, path, ans);
        path.pop();
    }
}

// 枚举每个位置的所有可能
pub struct Solution2;
impl Solution2 {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = Vec::with_capacity(16);
        let mut path = Vec::with_capacity(nums.len());
        Self::dfs(0, &nums, &mut path, &mut ans);
        ans
    }

    fn dfs(start: usize, nums: &[i32], path: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
        ans.push(path.clone());
        for i in start..nums.len() {
            path.push(nums[i]);
            // 下一个数只能向前找，不能向后，否则结果集中会出现类似(i, j)和(j, i)这样重复的相同组合
            Self::dfs(i + 1, nums, path, ans);
            path.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
