/// [994. 腐烂的橘子](https://leetcode.cn/problems/rotting-oranges/)
pub struct Solution;
use std::collections::VecDeque;
impl Solution {
    // bfs，原地模拟
    // 0: 空；1: 新鲜；2: 腐烂
    pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut q = VecDeque::new();
        let mut fresh_cnt = 0;
        for (r, row) in grid.iter().enumerate() {
            for (c, &val) in row.iter().enumerate() {
                if val == 1 {
                    fresh_cnt += 1;
                } else if val == 2 {
                    q.push_back((r as i32, c as i32, 0));
                }
            }
        }
        if fresh_cnt == 0 {
            return 0;
        }

        let mut ans = 0;
        let (m, n) = (grid.len() as i32, grid[0].len() as i32);
        while let Some((r, c, step)) = q.pop_front() {
            ans = ans.max(step);
            for (dr, dc) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let new_r = r + dr;
                let new_c = c + dc;
                if new_r >= 0 && new_r < m && new_c >= 0 && new_c < n {
                    let (r_idx, c_idx) = (new_r as usize, new_c as usize);
                    if grid[r_idx][c_idx] == 1 {
                        fresh_cnt -= 1;
                        grid[r_idx][c_idx] = 2;
                        q.push_back((new_r, new_c, step + 1));
                    }
                }
            }
        }

        if fresh_cnt == 0 { ans } else { -1 }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
