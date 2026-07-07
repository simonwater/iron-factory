/// # [42. 接雨水](https://leetcode.cn/problems/trapping-rain-water/)
///

pub struct Solution;

impl Solution {
    //
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.len() <= 2 {
            return 0;
        }
        let mut left_max = 0;
        let mut right_max = 0;
        let mut l = 0;
        let mut r = height.len() - 1;
        let mut ans = 0;
        while l < r {
            left_max = left_max.max(height[l]);
            right_max = right_max.max(height[r]);
            if left_max < right_max {
                ans += left_max - height[l];
                l += 1;
            } else {
                ans += right_max - height[r];
                r -= 1;
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
