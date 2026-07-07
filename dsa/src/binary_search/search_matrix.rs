/// [74. 搜索二维矩阵](https://leetcode.cn/problems/search-a-2d-matrix/)
pub struct Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix.is_empty() || matrix[0].is_empty() {
            return false;
        }
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut lo = 0;
        let mut hi = m * n - 1;
        while lo <= hi {
            let mid = lo + ((hi - lo) >> 1);
            let mid_val = matrix[mid / n][mid % n];
            if mid_val == target {
                return true;
            } else if mid_val < target {
                lo = mid + 1;
            } else {
                if mid == 0 {
                    break;
                }
                hi = mid - 1;
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
