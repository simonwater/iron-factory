//! [312. 戳气球](https://leetcode.cn/problems/burst-balloons/)
//!
//! 逆向思考，一个个戳气球反过来就是把气球一个个放回来

pub struct Solution;

/// 记忆化搜索；dfs(l, r): 把气球放回区间(l, r)内能获得的最大硬币数。
/// 枚举(l, r)之间的每一个位置 i ，把气球放在 i 获得的硬币就是 cur_val = nums[l] * nums[r] * nums[i]，
/// 对应到戳气球操作就是 i 位置的气球最后戳破。
/// 然后便有递归方程：dfs(l, r) = MAX {dfs(l, i) + dfs(i, r) + cur_val} (l < i && i < r)
impl Solution {
    pub fn max_coins(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut arr = vec![1; n + 2];
        arr[1..n + 1].copy_from_slice(&nums); // 增加左右边界
        let mut memo: Vec<Vec<i32>> = vec![vec![-1; n + 2]; n + 2];
        Self::dfs(&arr, 0, n + 1, &mut memo)
    }

    fn dfs(nums: &[i32], l: usize, r: usize, memo: &mut Vec<Vec<i32>>) -> i32 {
        if r - l == 1 {
            return 0;
        }
        if memo[l][r] != -1 {
            return memo[l][r];
        }
        let mut ans = 0;
        for i in l + 1..r {
            let cur = nums[l] * nums[r] * nums[i];
            let l_val = Self::dfs(nums, l, i, memo);
            let r_val = Self::dfs(nums, i, r, memo);
            ans = ans.max(l_val + r_val + cur);
        }
        memo[l][r] = ans;
        ans
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
