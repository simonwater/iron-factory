/// # [137. 只出现一次的数字 II](https://leetcode.cn/problems/single-number-ii/)
///
pub struct Solution;

impl Solution {
    //
    pub fn single_number(mut nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        for i in 0..32 {
            let mut acc = 0;
            for &num in &nums {
                acc += (num >> i) & 1;
            }
            ans |= ((acc % 3) & 1) << i;
        }
        ans
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
