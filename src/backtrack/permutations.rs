/// [46. 全排列](https://leetcode.cn/problems/permutations/)
struct Solution;

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = Vec::with_capacity(10);
        let mut path = Vec::with_capacity(nums.len());
        Self::dfs(&nums, &mut path, &mut ans);
        ans
    }

    fn dfs(nums: &[i32], path: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
        if path.len() == nums.len() {
            ans.push(path.clone());
            return;
        }
        for &num in nums {
            if !path.contains(&num) {
                path.push(num);
                Self::dfs(nums, path, ans);
                path.pop();
            }
        }
    }
}

struct Solution2;

// 用标志位判断是否已经用过，节约空间
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
