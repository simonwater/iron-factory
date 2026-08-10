//! [73. 矩阵置零](https://leetcode.cn/problems/set-matrix-zeroes/)
//!

pub struct Solution;

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut col_masks = [0u128, 0u128];
        let mut row_masks = [0u128, 0u128];
        for (i, row) in matrix.iter().enumerate() {
            for (j, &val) in row.iter().enumerate() {
                if val == 0 {
                    col_masks[j / 128] |= 1 << (j % 128);
                    row_masks[i / 128] |= 1 << (i % 128);
                }
            }
        }

        for i in 0..m {
            for j in 0..n {
                let row_hit = row_masks[i / 128] & (1u128 << (i % 128)) != 0;
                let col_hit = col_masks[j / 128] & (1u128 << (j % 128)) != 0;
                if row_hit || col_hit {
                    matrix[i][j] = 0;
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
