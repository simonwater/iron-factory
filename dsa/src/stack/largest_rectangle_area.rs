//! [84. 柱状图中最大的矩形](https://leetcode.cn/problems/largest-rectangle-in-histogram//)
//!

/// 枚举每个柱子，计算以其为高的矩形面积，并记录最大值。宽度取决于往左第一根比他矮的柱子和往右第一根比他矮的柱子，
/// 两根矮柱之间的内距就是当前的宽度。
///
/// 常规枚举法往一个方向找矮柱时，需要不断的枚举当前柱子 A 前面更高的柱子，直到找到更矮的 B ，这时我们知道 B 肯定
/// 比 A 小，且 A，B间的都比A B高或者等于 A，很容易便能看到，对于 A 以后的柱子来说，中间这些更高的柱子肯定不会成为他们
/// 要找的矮柱，如果他们比A高，A会成为他们要找的矮柱，他们小于或等于A，那么 B 或者B前面更矮的会成为他们要找的结果。
/// 结论就是中间这些可以直接删掉，就能提升查找效率。这时候维护一个单调栈就是很自然的选择。
///
pub struct Solution;

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let n = heights.len();
        if n == 1 {
            return heights[0];
        }
        let mut stack: Vec<(i32, i32)> = Vec::with_capacity(n + 1);
        let mut rights = vec![0; n];
        stack.push((n as i32, -1));
        for i in (0..n).rev() {
            let cur_h = heights[i];
            while let Some(&(idx, h)) = stack.last() {
                if cur_h <= h {
                    stack.pop();
                } else {
                    rights[i] = idx;
                    break;
                }
            }
            stack.push((i as i32, cur_h));
        }

        let mut ans = 0;
        stack.clear();
        stack.push((-1, -1));
        for i in 0..n {
            let cur_h = heights[i];
            while let Some(&(left_idx, h)) = stack.last() {
                if cur_h <= h {
                    stack.pop();
                } else {
                    let right_idx = rights[i];
                    ans = ans.max((right_idx - left_idx - 1) * cur_h);
                    break;
                }
            }
            stack.push((i as i32, cur_h));
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
