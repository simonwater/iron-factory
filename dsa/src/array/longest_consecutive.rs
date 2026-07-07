/// [128. 最长连续序列](https://leetcode.cn/problems/longest-consecutive-sequence/)
pub struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        if nums.len() <= 1 {
            return nums.len() as i32;
        }
        let set = nums.into_iter().collect::<HashSet<i32>>();
        let mut ans = 1;
        for &num in set.iter() {
            if set.contains(&(num - 1)) {
                continue;
            }
            let mut next = num + 1;
            let mut cur_len = 1;
            while set.contains(&next) {
                cur_len += 1;
                next += 1;
            }
            if cur_len > ans {
                ans = cur_len;
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
