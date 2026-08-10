//! [48. 旋转图像](https://leetcode.cn/problems/rotate-image/)
//!

pub struct Solution;

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        for i in 1..n {
            for j in n - i..n {
                let r = n - 1 - j;
                let c = n - 1 - i;
                (matrix[i][j], matrix[r][c]) = (matrix[r][c], matrix[i][j]);
            }
        }

        for i in 0..n / 2 {
            matrix.swap(i, n - 1 - i);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
