//! [873. 最长的斐波那契子序列的长度](https://leetcode.cn/problems/length-of-longest-fibonacci-subsequence/)
//!
//! 不同于常规的序列型动态规划，常规的状态方程描述的是以某个元素为结尾时的最优解，该题状态方程定义的是以某两个元素
//! 为结尾时的最优解

pub struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn len_longest_fib_subseq(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut map = HashMap::with_capacity(n);
        for (i, &num) in arr.iter().enumerate() {
            map.insert(num, i);
        }
        let mut dp = vec![vec![0; n]; n];
        let mut ans = 0;
        for j in 0..n {
            for i in (0..j).rev() {
                let prev = arr[j] - arr[i];
                if prev >= arr[i] {
                    break;
                }
                if let Some(&prev_idx) = map.get(&prev) {
                    let prev_len = dp[prev_idx][i];
                    dp[i][j] = if prev_len == 0 { 3 } else { prev_len + 1 };
                    ans = ans.max(dp[i][j]);
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
