/// [46. 全排列](https://leetcode.cn/problems/permutations/)
struct Solution;

/// 枚举每个位置的所有可能填充数字
impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = Vec::with_capacity(10);
        let mut path = Vec::with_capacity(nums.len());
        let mut used = vec![false; nums.len()];
        Self::dfs(&nums, &mut path, &mut ans, &mut used);
        ans
    }

    fn dfs(nums: &[i32], path: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>, used: &mut Vec<bool>) {
        if path.len() == nums.len() {
            ans.push(path.clone());
            return;
        }
        for (idx, &num) in nums.iter().enumerate() {
            if !used[idx] {
                used[idx] = true;
                path.push(num);
                Self::dfs(nums, path, ans, used);
                path.pop();
                used[idx] = false;
            }
        }
    }
}

struct Solution2;

/// 用标志位判断是否已经用过，节约空间
impl Solution2 {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = Vec::with_capacity(10);
        let mut path = Vec::with_capacity(nums.len());
        Self::dfs(&nums, &mut path, &mut ans, 0);
        ans
    }

    fn dfs(nums: &[i32], path: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>, used_mask: i32) {
        if path.len() == nums.len() {
            ans.push(path.clone());
            return;
        }
        for (idx, &num) in nums.iter().enumerate() {
            if used_mask & (1 << idx) == 0 {
                path.push(num);
                Self::dfs(nums, path, ans, used_mask | (1 << idx));
                path.pop();
            }
        }
    }
}

/// 原地交换法，通过交换来枚举每个位置所有可能的数字
struct Solution3;
impl Solution3 {
    pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = Vec::with_capacity(10);
        Self::dfs(0, &mut nums, &mut ans);
        ans
    }

    fn dfs(start: usize, nums: &mut [i32], ans: &mut Vec<Vec<i32>>) {
        if start == nums.len() {
            ans.push(nums.to_vec());
        }
        for i in start..nums.len() {
            // 从start到n - 1每个位置上的数字都轮流来填充位置start
            nums.swap(start, i);
            // start及其左边的部分已经固定，接下来让剩余的数字递归决定位置start + 1.
            Self::dfs(start + 1, nums, ans);
            // 恢复现场，准备让一下个数来填充
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
