/// # [137. 只出现一次的数字 II](https://leetcode.cn/problems/single-number-ii/)
/// 只出现 1 次的数和出现 3 次的数，分别统计所有数字相同位的元素，对 3 取模，出现 3 次的被消灭，剩下的余数为结果
pub struct Solution;

impl Solution {
    //
    pub fn single_number(nums: Vec<i32>) -> i32 {
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
