/// [33. 搜索旋转排序数组](https://leetcode.cn/problems/search-in-rotated-sorted-array/)
pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {
            return -1;
        }
        let mut lo = 0;
        let mut hi = nums.len() - 1;
        let first = nums[lo]; // 避免每次都重复读
        let last = nums[hi];
        while lo <= hi {
            let mid = lo + ((hi - lo) >> 1);
            if nums[mid] == target {
                return mid as i32;
            }
            if nums[mid] >= nums[0] {
                // mid 左侧有序
                if target >= first && target < nums[mid] {
                    //
                    hi = mid - 1;
                } else {
                    lo = mid + 1;
                }
            } else {
                // mid右侧有序
                if target > nums[mid] && target <= last {
                    lo = mid + 1;
                } else {
                    if mid == 0 {
                        return -1;
                    }
                    hi = mid - 1;
                }
            }
        }
        -1
    }

    pub fn find_min(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            unreachable!();
        }
        let mut lo = 0;
        let mut hi = nums.len() - 1;
        let first = nums[lo];
        if first <= nums[hi] {
            return first;
        }
        lo = 1; // nums[0]肯定不是结果，直接从1开始能避免后续的溢出或者越界判断。

        while lo <= hi {
            let mid = lo + ((hi - lo) >> 1);
            if nums[mid] < nums[mid - 1] {
                return nums[mid];
            }
            if nums[mid] >= first {
                lo = mid + 1;
            } else {
                hi = mid - 1;
            }
        }

        unreachable!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        let target = 0;
        let ans = 4;
        assert_eq!(Solution::search(nums, target), ans);

        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        let target = 3;
        let ans = -1;
        assert_eq!(Solution::search(nums, target), ans);

        let nums = vec![1];
        let target = 0;
        let ans = -1;
        assert_eq!(Solution::search(nums, target), ans);
    }

    #[test]
    fn test2() {
        let nums = vec![3, 4, 5, 1, 2];
        let ans = 1;
        assert_eq!(Solution::find_min(nums), ans);

        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        let ans = 0;
        assert_eq!(Solution::find_min(nums), ans);

        let nums = vec![11, 13, 15, 17];
        let ans = 11;
        assert_eq!(Solution::find_min(nums), ans);

        let nums = vec![1];
        let ans = 1;
        assert_eq!(Solution::find_min(nums), ans);
    }
}
