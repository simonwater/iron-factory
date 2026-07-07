/// # [221. 最大正方形](https://leetcode.cn/problems/maximal-square/)
/// 找到只包含1的最大正方形的面积

pub struct Solution;

impl Solution {
    //
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut dp = vec![vec![0; n]; m];
        let mut ans = 0;
        for (r, row) in matrix.iter().enumerate() {
            for (c, &val) in row.iter().enumerate() {
                if val == '0' {
                    continue;
                }
                if r == 0 || c == 0 {
                    dp[r][c] = 1;
                } else {
                    dp[r][c] = dp[r - 1][c].min(dp[r][c - 1].min(dp[r - 1][c - 1])) + 1;
                }
                ans = ans.max(dp[r][c]);
            }
        }

        ans * ans
    }
}

pub struct Solution2;

impl Solution2 {
    // 空间压缩，重点关注左上角值的滚动。当前值跟新前，是自己的top，也是下一个值的left-top，而更新以后则成为下一个值的left。
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let n = matrix[0].len();
        let mut dp = vec![0; n + 1];
        let mut ans = 0;
        for (_, row) in matrix.iter().enumerate() {
            let mut prev = 0;
            for (c, &val) in row.iter().enumerate() {
                let i = c + 1; // dp数组索引
                if val == '1' {
                    let l = dp[i - 1];
                    let t = dp[i];
                    dp[i] = l.min(t.min(prev)) + 1;
                    prev = t;
                    ans = ans.max(dp[i]);
                } else {
                    dp[i] = 0;
                    prev = 0;
                }
            }
        }

        ans * ans
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
