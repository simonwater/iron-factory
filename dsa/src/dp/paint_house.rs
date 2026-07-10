//! [LCR 091. 粉刷房子](https://leetcode.cn/problems/JEj789/)
//!
//! **状态机DP**

pub struct Solution;

impl Solution {
    pub fn min_cost(costs: Vec<Vec<i32>>) -> i32 {
        let (mut color0, mut color1, mut color2) = (0, 0, 0);
        for cost in costs {
            (color0, color1, color2) = (
                cost[0] + color1.min(color2),
                cost[1] + color0.min(color2),
                cost[2] + color0.min(color1),
            );
        }
        color0.min(color1.min(color2))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
