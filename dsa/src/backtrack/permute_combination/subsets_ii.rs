/// [90. 子集 II](https://leetcode.cn/problems/subsets-ii/)
pub struct Solution;

impl Solution {
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = Vec::with_capacity(16);
        let mut path = Vec::with_capacity(10);
        nums.sort_unstable();
        Self::dfs(0, &nums, &mut path, &mut ans);
        ans
    }

    fn dfs(start: usize, nums: &[i32], path: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
        ans.push(path.clone());

        for i in start..nums.len() {
            // 只要当前的数字在path现在需要填充的位置上已经使用过了，就进行剪枝
            if i > start && nums[i] == nums[i - 1] {
                continue;
            }
            path.push(nums[i]);
            Self::dfs(i + 1, nums, path, ans);
            path.pop();
        }
    }
}

pub struct Solution2;
impl Solution2 {
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = Vec::with_capacity(16);
        let mut path = Vec::with_capacity(10);
        nums.sort_unstable();
        Self::dfs(0, &nums, &mut path, &mut ans);
        ans
    }

    fn dfs(mut i: usize, nums: &[i32], path: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
        if i == nums.len() {
            ans.push(path.clone());
            return;
        }
        path.push(nums[i]);
        Self::dfs(i + 1, nums, path, ans);
        path.pop();
        // 对于当前层，也就是path现在要填充的位置j，使用数字nums[i]的情况已经在刚才处理完
        // 不选nums[i]的分支进入下一层后，如果遇到相同数字，继续往j处填充，就会重复处理。
        while i < nums.len() - 1 && nums[i] == nums[i + 1] {
            i += 1;
        }
        Self::dfs(i + 1, nums, path, ans);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
