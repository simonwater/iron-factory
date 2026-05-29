/// [11. 盛最多水的容器](https://leetcode.cn/problems/container-with-most-water/)
struct Solution;

impl Solution {
    pub fn max_area(heights: Vec<i32>) -> i32 {
        if heights.len() < 1 {
            return 0;
        }
        let mut ans = 0;
        let mut left = 0;
        let mut right = heights.len() - 1;
        while left < right {
            let h = heights[left].min(heights[right]);
            ans = ans.max(h * (right - left) as i32);
            if heights[left] < heights[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
