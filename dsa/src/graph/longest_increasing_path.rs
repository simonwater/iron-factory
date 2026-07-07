/// # [329. 矩阵中的最长递增路径](https://leetcode.cn/problems/longest-increasing-path-in-a-matrix/)
///

pub struct Solution;

impl Solution {
    //
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut dp = vec![vec![0; n]; m];
        let mut ans = 0;
        for i in 0..m {
            for j in 0..n {
                if dp[i][j] == 0 {
                    ans = ans.max(Self::dfs(i, j, &matrix, &mut dp));
                }
            }
        }

        ans
    }

    fn dfs(r: usize, c: usize, matrix: &Vec<Vec<i32>>, dp: &mut Vec<Vec<i32>>) -> i32 {
        if dp[r][c] != 0 {
            return dp[r][c];
        }
        let m = matrix.len();
        let n = matrix[0].len();
        let cur = matrix[r][c];
        let mut ans = 0;
        if r > 0 && cur < matrix[r - 1][c] {
            ans = ans.max(Self::dfs(r - 1, c, matrix, dp));
        }
        if c > 0 && cur < matrix[r][c - 1] {
            ans = ans.max(Self::dfs(r, c - 1, matrix, dp));
        }
        if r < m - 1 && cur < matrix[r + 1][c] {
            ans = ans.max(Self::dfs(r + 1, c, matrix, dp));
        }
        if c < n - 1 && cur < matrix[r][c + 1] {
            ans = ans.max(Self::dfs(r, c + 1, matrix, dp));
        }

        ans += 1;
        dp[r][c] = ans;
        ans
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
