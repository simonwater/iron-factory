/// [1091. 二进制矩阵中的最短路径](https://leetcode.cn/problems/shortest-path-in-binary-matrix/)
struct Solution;

use std::collections::VecDeque;

const D: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];
impl Solution {
    // bfs层序遍历
    pub fn shortest_path_binary_matrix(mut grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        if grid[0][0] == 1 || grid[n - 1][n - 1] == 1 {
            return -1;
        }
        if n == 1 {
            return 1;
        }

        let mut q = VecDeque::with_capacity(16);
        grid[0][0] = 1;
        q.push_back((0, 0, 1));
        let n_i32 = n as i32;
        while let Some((r, c, dist)) = q.pop_front() {
            for (dr, dc) in D {
                let new_r = r + dr;
                let new_c = c + dc;
                if new_r >= 0 && new_r < n_i32 && new_c >= 0 && new_c < n_i32 {
                    let new_dist = dist + 1;
                    if new_r == n_i32 - 1 && new_c == n_i32 - 1 {
                        return new_dist;
                    }
                    let r_idx = new_r as usize;
                    let c_idx = new_c as usize;
                    if grid[r_idx][c_idx] == 1 {
                        continue;
                    }

                    grid[r_idx][c_idx] = 1;
                    q.push_back((new_r, new_c, new_dist));
                }
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
