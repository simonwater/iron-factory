//! [240. 搜索二维矩阵 II](https://leetcode.cn/problems/search-a-2d-matrix-ii/)
//!

pub struct Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut r = 0;
        let mut c = n - 1;
        while r < m {
            if matrix[r][c] == target {
                return true;
            } else if matrix[r][c] < target {
                r += 1;
            } else {
                if c == 0 {
                    return false;
                }
                c -= 1;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
