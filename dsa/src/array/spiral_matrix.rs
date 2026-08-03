//! [54. 螺旋矩阵](https://leetcode.cn/problems/spiral-matrix//)
//!

pub struct Solution;

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let (m, n) = (matrix.len(), matrix[0].len());
        let len = m * n;
        let mut ans = Vec::with_capacity(len);
        let (mut l, mut t, mut r, mut b) = (0, 0, n as i32 - 1, m as i32 - 1);
        while ans.len() < len {
            for col in l..=r {
                ans.push(matrix[t as usize][col as usize]);
            }
            t += 1;

            for row in t..=b {
                ans.push(matrix[row as usize][r as usize]);
            }
            r -= 1;

            // 判断是否还有合法的数据
            if t <= b {
                for col in (l..=r).rev() {
                    ans.push(matrix[b as usize][col as usize]);
                }
                b -= 1;
            }

            if l <= r {
                for row in (t..=b).rev() {
                    ans.push(matrix[row as usize][l as usize]);
                }
                l += 1;
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
