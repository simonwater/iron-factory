/// [200. 岛屿数量](https://leetcode.cn/problems/number-of-islands/)

/// dfs
pub struct Solution;
impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let mut ans = 0;
        for r in 0..grid.len() {
            for c in 0..grid[0].len() {
                // 发现一个，消灭一片（dfs/bfs）
                if grid[r][c] == '1' {
                    ans += 1;
                    Self::dfs(&mut grid, r as i32, c as i32);
                }
            }
        }
        ans
    }

    fn dfs(grid: &mut Vec<Vec<char>>, r: i32, c: i32) {
        let (m, n) = (grid.len() as i32, grid[0].len() as i32);
        if r < 0 || r == m || c < 0 || c == n {
            return;
        }
        let (r_idx, c_idx) = (r as usize, c as usize);
        if grid[r_idx][c_idx] == '1' {
            grid[r_idx][c_idx] = '0';
            Self::dfs(grid, r + 1, c);
            Self::dfs(grid, r - 1, c);
            Self::dfs(grid, r, c + 1);
            Self::dfs(grid, r, c - 1);
        }
    }
}

/// bfs
pub struct Solution2;
use std::collections::VecDeque;
impl Solution2 {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let mut ans = 0;
        for r in 0..grid.len() {
            for c in 0..grid[0].len() {
                // 发现一个，消灭一片（dfs/bfs）
                if grid[r][c] == '1' {
                    ans += 1;
                    Self::bfs(&mut grid, r as i32, c as i32);
                }
            }
        }
        ans
    }

    fn bfs(grid: &mut Vec<Vec<char>>, r: i32, c: i32) {
        let (m, n) = (grid.len() as i32, grid[0].len() as i32);
        let mut q = VecDeque::new();
        grid[r as usize][c as usize] = '0';
        q.push_back((r, c));
        while let Some((r, c)) = q.pop_front() {
            for (dr, dc) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let new_r = r + dr;
                let new_c = c + dc;
                if new_r >= 0 && new_r < m && new_c >= 0 && new_c < n {
                    let (r_idx, c_idx) = (new_r as usize, new_c as usize);
                    if grid[r_idx][c_idx] == '1' {
                        grid[r_idx][c_idx] = '0';
                        q.push_back((new_r, new_c));
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
