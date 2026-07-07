/// # [136. 只出现一次的数字](https://leetcode.cn/problems/single-number/description/)
///
/// 异或性质：1.相同的数异或结果为0；2. 任意一个数和0异或结果是这个数本身
///   1 ^ 0 = 1
///   0 ^ 1 = 1
///   0 ^ 0 = 0
///   1 ^ 1 = 0
pub struct Solution;

impl Solution {
    //
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut iter = nums.into_iter();
        let mut ans = iter.next().unwrap();
        for num in iter {
            ans = ans ^ num;
        }
        ans
    }
}

pub struct Solution2;

impl Solution2 {
    //
    pub fn single_number(nums: Vec<i32>) -> i32 {
        nums.iter().fold(0, |acc, &num| acc ^ num)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
