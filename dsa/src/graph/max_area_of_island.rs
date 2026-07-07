/// [695. 岛屿的最大面积](https://leetcode.cn/problems/max-area-of-island/)
pub struct Solution;

impl Solution {
    pub fn max_area_of_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        for r in 0..grid.len() {
            for c in 0..grid[0].len() {
                if grid[r][c] == 1 {
                    ans = ans.max(Self::dfs(&mut grid, r as i32, c as i32));
                }
            }
        }
        ans
    }

    fn dfs(grid: &mut Vec<Vec<i32>>, r: i32, c: i32) -> i32 {
        let m = grid.len() as i32;
        let n = grid[0].len() as i32;
        if r < 0 || r == m || c < 0 || c == n {
            return 0;
        }
        let r_idx = r as usize;
        let c_idx = c as usize;
        if grid[r_idx][c_idx] == 0 {
            return 0;
        }
        grid[r_idx][c_idx] = 0;
        let mut ans = 1;
        for (dr, dc) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            ans += Self::dfs(grid, r + dr, c + dc);
        }
        ans
    }
}

pub struct Solution2;
use std::collections::VecDeque;
impl Solution2 {
    pub fn max_area_of_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        for r in 0..grid.len() {
            for c in 0..grid[0].len() {
                if grid[r][c] == 1 {
                    ans = ans.max(Self::bfs(&mut grid, r as i32, c as i32));
                }
            }
        }
        ans
    }

    fn bfs(grid: &mut Vec<Vec<i32>>, r: i32, c: i32) -> i32 {
        let m = grid.len() as i32;
        let n = grid[0].len() as i32;
        let mut q = VecDeque::new();
        grid[r as usize][c as usize] = 0;
        q.push_back((r, c));
        let mut ans = 1;
        while let Some((r, c)) = q.pop_front() {
            for (dr, dc) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let r_next = r + dr;
                let c_next = c + dc;
                if r_next >= 0 && r_next < m && c_next >= 0 && c_next < n {
                    let r_idx = r_next as usize;
                    let c_idx = c_next as usize;
                    if grid[r_idx][c_idx] == 0 {
                        continue;
                    }
                    ans += 1;
                    grid[r_idx][c_idx] = 0;
                    q.push_back((r_next, c_next));
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
