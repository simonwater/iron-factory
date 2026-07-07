/// [977. 有序数组的平方](https://leetcode.cn/problems/squares-of-a-sorted-array/description/)
pub struct Solution;

impl Solution {
    pub fn sorted_squares_simple(nums: Vec<i32>) -> Vec<i32> {
        let mut ans: Vec<i32> = nums.iter().map(|x| x * x).collect();
        ans.sort_unstable();
        ans
    }

    pub fn sorted_squares(mut nums: Vec<i32>) -> Vec<i32> {
        for x in nums.iter_mut() {
            *x = (*x) * (*x);
        }
        let n = nums.len();
        let mut ans = vec![0; n];
        let (mut l, mut r, mut i) = (0, n, n); // usize自减不能到负数
        while i > 0 {
            if nums[l] > nums[r - 1] {
                ans[i - 1] = nums[l];
                l += 1;
            } else {
                ans[i - 1] = nums[r - 1];
                r -= 1;
            }
            i -= 1;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![-4, -1, 0, 3, 10];
        let ans = vec![0, 1, 9, 16, 100];
        assert_eq!(Solution::sorted_squares_simple(nums), ans);

        let nums = vec![-7, -3, 2, 3, 11];
        let ans = vec![4, 9, 9, 49, 121];
        assert_eq!(Solution::sorted_squares_simple(nums), ans);
    }

    #[test]
    fn test2() {
        let nums = vec![-4, -1, 0, 3, 10];
        let ans = vec![0, 1, 9, 16, 100];
        assert_eq!(Solution::sorted_squares(nums), ans);

        let nums = vec![-7, -3, 2, 3, 11];
        let ans = vec![4, 9, 9, 49, 121];
        assert_eq!(Solution::sorted_squares(nums), ans);

        let nums = vec![1];
        let ans = vec![1];
        assert_eq!(Solution::sorted_squares(nums), ans);

        let nums = vec![-3, -2, -1];
        let ans = vec![1, 4, 9];
        assert_eq!(Solution::sorted_squares(nums), ans);

        let nums = vec![1, 2, 3];
        let ans = vec![1, 4, 9];
        assert_eq!(Solution::sorted_squares(nums), ans);
    }
}
