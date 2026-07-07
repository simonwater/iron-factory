/// [238. 除了自身以外数组的乘积](https://leetcode.cn/problems/product-of-array-except-self/description/)
pub struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut ans = vec![1; n];
        for i in (0..n - 1).rev() {
            ans[i] = nums[i + 1] * ans[i + 1];
        }

        let mut p = 1;
        for i in 0..n {
            ans[i] = p * ans[i];
            p = p * nums[i];
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![1, 2, 3, 4];
        let ans = vec![24, 12, 8, 6];
        assert_eq!(Solution::product_except_self(nums), ans);

        let nums = vec![-1, 1, 0, -3, 3];
        let ans = vec![0, 0, 9, 0, 0];
        assert_eq!(Solution::product_except_self(nums), ans);
    }
}
