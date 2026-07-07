pub struct Solution;

impl Solution {
    // 快速排序
    pub fn sort(mut nums: Vec<i32>) -> Vec<i32> {
        if nums.is_empty() {
            return nums;
        }
        let n = nums.len();
        Self::qsort(&mut nums, 0, n - 1);
        nums
    }

    fn qsort(nums: &mut [i32], start: usize, end: usize) {
        if start >= end {
            return;
        }
        let p = Self::partition(nums, start, end);
        if p > 0 {
            Self::qsort(nums, start, p - 1);
        }
        Self::qsort(nums, p + 1, end);
    }

    fn partition(nums: &mut [i32], start: usize, end: usize) -> usize {
        let pivot = nums[end];
        let mut i = start;
        for j in start..=end {
            if nums[j] <= pivot {
                nums.swap(i, j);
                i += 1;
            }
        }
        i - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![4, 2, 1, 5, 6, 3];
        let ans = vec![1, 2, 3, 4, 5, 6];
        assert_eq!(Solution::sort(nums), ans);

        let nums = vec![3, 2, 3, 1, 2, 4, 5, 5, 6];
        let ans = vec![1, 2, 2, 3, 3, 4, 5, 5, 6];
        assert_eq!(Solution::sort(nums), ans);

        let nums = vec![2, 1];
        let ans = vec![1, 2];
        assert_eq!(Solution::sort(nums), ans);

        let nums = vec![1];
        let ans = vec![1];
        assert_eq!(Solution::sort(nums), ans);

        let nums = vec![1, 1, 1];
        let ans = vec![1, 1, 1];
        assert_eq!(Solution::sort(nums), ans);
    }
}
