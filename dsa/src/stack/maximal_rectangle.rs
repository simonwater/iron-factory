//! [85. 最大矩形](https://leetcode.cn/problems/maximal-rectangle/)
//!

pub struct Solution;

impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let n = matrix[0].len();
        let mut heights = vec![0; n + 1];
        let mut ans = 0;
        for row in matrix.iter() {
            for (i, &ch) in row.iter().enumerate() {
                if ch == '1' {
                    heights[i] += 1;
                } else {
                    heights[i] = 0;
                }
            }
            let cur_max_area = Self::max_area(&heights);
            ans = ans.max(cur_max_area);
        }

        ans
    }

    fn max_area(heights: &[i32]) -> i32 {
        let n = heights.len();
        let mut ans = 0;
        let mut stack = Vec::with_capacity(n);
        for i in 0..n {
            let cur_h = heights[i];
            while let Some(&top_idx) = stack.last() {
                let top_h = heights[top_idx];
                if top_h >= cur_h {
                    stack.pop();
                    let w = if let Some(&pre_idx) = stack.last() {
                        (i - pre_idx - 1) as i32
                    } else {
                        i as i32
                    };
                    ans = ans.max(top_h * w);
                } else {
                    break;
                }
            }
            stack.push(i);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
