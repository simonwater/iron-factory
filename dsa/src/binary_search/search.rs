/// [704. 二分查找](https://leetcode.cn/problems/binary-search/)
pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {
            // 防止usize为-1
            return -1;
        }
        let mut lo = 0;
        let mut hi = nums.len() - 1;
        while lo <= hi {
            // (left + right) / 2 ：left + right 可能超过 usize::MAX
            // 直接触发cpu位移指令，比 / 2 更快
            let mid = lo + ((hi - lo) >> 2);
            if nums[mid] == target {
                return mid as i32;
            } else if nums[mid] < target {
                lo = mid + 1;
            } else {
                if mid == 0 {
                    // 防止usize为-1
                    return -1;
                }
                hi = mid - 1;
            }
        }
        -1
    }

    pub fn search2(nums: Vec<i32>, target: i32) -> i32 {
        let mut lo = 0isize;
        let mut hi = nums.len() as isize - 1;
        while lo <= hi {
            let mid = lo + ((hi - lo) >> 1);
            let mid_idx = mid as usize;
            if nums[mid_idx] == target {
                return mid as i32;
            } else if nums[mid_idx] < target {
                lo = mid + 1;
            } else {
                hi = mid - 1;
            }
        }
        -1
    }

    pub fn search3(nums: Vec<i32>, target: i32) -> i32 {
        let mut lo = 0;
        let mut hi = nums.len();
        while lo < hi {
            let mid = lo + ((hi - lo) >> 1);
            if nums[mid] == target {
                return mid as i32;
            } else if nums[mid] < target {
                lo += 1;
            } else {
                hi = mid;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![-1, 0, 3, 5, 9, 12];
        let target = 9;
        let ans = 4;
        assert_eq!(Solution::search(nums, target), ans);

        let nums = vec![-1, 0, 3, 5, 9, 12];
        let target = 2;
        let ans = -1;
        assert_eq!(Solution::search(nums, target), ans);

        let nums = vec![1];
        let target = -5;
        let ans = -1;
        assert_eq!(Solution::search(nums, target), ans);

        let nums = vec![];
        let target = 1;
        let ans = -1;
        assert_eq!(Solution::search(nums, target), ans);
    }

    #[test]
    fn test2() {
        let nums = vec![-1, 0, 3, 5, 9, 12];
        let target = 9;
        let ans = 4;
        assert_eq!(Solution::search2(nums, target), ans);

        let nums = vec![-1, 0, 3, 5, 9, 12];
        let target = 2;
        let ans = -1;
        assert_eq!(Solution::search2(nums, target), ans);

        let nums = vec![1];
        let target = -5;
        let ans = -1;
        assert_eq!(Solution::search2(nums, target), ans);

        let nums = vec![];
        let target = 1;
        let ans = -1;
        assert_eq!(Solution::search2(nums, target), ans);
    }

    #[test]
    fn test3() {
        let nums = vec![-1, 0, 3, 5, 9, 12];
        let target = 9;
        let ans = 4;
        assert_eq!(Solution::search3(nums, target), ans);

        let nums = vec![-1, 0, 3, 5, 9, 12];
        let target = 2;
        let ans = -1;
        assert_eq!(Solution::search3(nums, target), ans);

        let nums = vec![1];
        let target = -5;
        let ans = -1;
        assert_eq!(Solution::search3(nums, target), ans);

        let nums = vec![];
        let target = 1;
        let ans = -1;
        assert_eq!(Solution::search3(nums, target), ans);
    }
}
