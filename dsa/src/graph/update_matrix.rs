/// [542. 01 矩阵](https://leetcode.cn/problems/01-matrix/)
/// 求所有元素到最近的0的距离
use std::collections::VecDeque;

/// 正向搜索，时间复杂度O(M^2*N^2)
pub struct Solution;
impl Solution {
    pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = mat.len();
        let n = mat[0].len();
        let mut ans = vec![vec![0; n]; m]; // m行n列
        for (r, row) in mat.iter().enumerate() {
            for (c, &val) in row.iter().enumerate() {
                if val == 1 {
                    ans[r][c] = Self::bfs(&mat, r, c);
                }
            }
        }
        ans
    }

    fn bfs(mat: &Vec<Vec<i32>>, r: usize, c: usize) -> i32 {
        let m = mat.len();
        let n = mat[0].len();
        let mut q = VecDeque::new();
        let mut visited = vec![vec![false; n]; m];
        let (m, n) = (m as i32, n as i32);
        visited[r][c] = true;
        q.push_back((r as i32, c as i32, 0));
        while let Some((r, c, step)) = q.pop_front() {
            for (dr, dc) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let (new_r, new_c) = (r + dr, c + dc);
                if new_r >= 0 && new_r < m && new_c >= 0 && new_c < n {
                    let (r_idx, c_idx) = (new_r as usize, new_c as usize);
                    if visited[r_idx][c_idx] {
                        continue;
                    }
                    if mat[r_idx][c_idx] == 0 {
                        return step + 1;
                    }
                    visited[r_idx][c_idx] = true;
                    q.push_back((new_r, new_c, step + 1));
                }
            }
        }
        -1
    }
}

/// 反向从0开始查找，所有0全部入队列，在搜索图中处于同一层，然后从这一层扩散到整个图，遇到1时进行松弛操作。
/// 时间复杂度O(M*N)
pub struct Solution2;
impl Solution2 {
    pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (m, n) = (mat.len(), mat[0].len());
        let mut ans = vec![vec![0; n]; m];
        let mut q = VecDeque::new();
        for (r, row) in mat.iter().enumerate() {
            for (c, &val) in row.iter().enumerate() {
                if val == 0 {
                    q.push_back((r as i32, c as i32, 0));
                } else {
                    ans[r][c] = i32::MAX;
                }
            }
        }

        let (m_i32, n_i32) = (m as i32, n as i32);
        while let Some((r, c, step)) = q.pop_front() {
            for (dr, dc) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let new_r = r + dr;
                let new_c = c + dc;
                if new_r >= 0 && new_r < m_i32 && new_c >= 0 && new_c < n_i32 {
                    let (r_idx, c_idx) = (new_r as usize, new_c as usize);
                    if ans[r_idx][c_idx] > step + 1 {
                        ans[r_idx][c_idx] = step + 1;
                        q.push_back((new_r, new_c, step + 1));
                    }
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
