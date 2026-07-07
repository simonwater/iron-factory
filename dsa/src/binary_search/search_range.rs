/// [34. 在排序数组中查找元素的第一个和最后一个位置](https://leetcode.cn/problems/find-first-and-last-position-of-element-in-sorted-array/)
pub struct Solution;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let lo = Self::search_bound(&nums, target, true);
        if lo == -1 {
            return vec![-1, -1];
        }
        let hi = Self::search_bound(&nums, target, false);
        vec![lo, hi]
    }

    /// 1. usize需谨慎处理递减时的边界
    /// 2. 参数用切片类型而不是&Vec<i32>：
    ///   一是更灵活，调用方不仅可以传递动态数组，而且也能传定长数组以及数组的切片。
    ///   二是&Vec是一个指针的指针，而 &[i32] 胖指针直接包含了内存首地址和长度，切片少了一层寻址，对cpu缓存更友好
    fn search_bound(nums: &[i32], target: i32, is_lower: bool) -> i32 {
        if nums.is_empty() {
            return -1;
        }
        let mut lo = 0;
        let mut hi = nums.len() - 1;
        let mut ans = -1;
        while lo <= hi {
            let mid = lo + ((hi - lo) >> 1);
            if nums[mid] == target {
                ans = mid as i32;
                if is_lower {
                    if mid == 0 {
                        break;
                    }
                    hi = mid - 1;
                } else {
                    lo = mid + 1;
                }
            } else if nums[mid] < target {
                lo = mid + 1;
            } else {
                if mid == 0 {
                    break;
                }
                hi = mid - 1;
            }
        }
        ans
    }

    pub fn search_bound2(nums: &[i32], target: i32, is_lower: bool) -> i32 {
        let mut lo = 0isize;
        let mut hi = nums.len() as isize - 1;
        let mut ans = -1isize;
        while lo <= hi {
            let mid = lo + ((hi - lo) >> 1);
            let mid_idx = mid as usize;
            if nums[mid_idx] == target {
                ans = mid;
                if is_lower {
                    hi = mid - 1;
                } else {
                    lo = mid + 1;
                }
            } else if nums[mid_idx] < target {
                lo = mid + 1;
            } else {
                hi = mid - 1;
            }
        }
        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![5, 7, 7, 8, 8, 10];
        let target = 8;
        let ans = vec![3, 4];
        assert_eq!(Solution::search_range(nums, target), ans);

        let nums = vec![5, 7, 7, 8, 8, 10];
        let target = 6;
        let ans = vec![-1, -1];
        assert_eq!(Solution::search_range(nums, target), ans);

        let nums = vec![];
        let target = 0;
        let ans = vec![-1, -1];
        assert_eq!(Solution::search_range(nums, target), ans);

        let nums = vec![1];
        let target = 1;
        let ans = vec![0, 0];
        assert_eq!(Solution::search_range(nums, target), ans);

        let nums = vec![1];
        let target = 0;
        let ans = vec![-1, -1];
        assert_eq!(Solution::search_range(nums, target), ans);
    }
}
