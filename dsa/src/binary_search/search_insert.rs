/// [35. 搜索插入位置](https://leetcode.cn/problems/search-insert-position/)
pub struct Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut lo = 0;
        let mut hi = nums.len() - 1;
        while lo <= hi {
            let mid = lo + ((hi - lo) >> 1);
            if nums[mid] == target {
                return mid as i32;
            } else if nums[mid] < target {
                lo = mid + 1;
            } else {
                if mid == 0 {
                    return 0;
                }
                hi = mid - 1;
            }
        }
        lo as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![1, 3, 5, 6];
        let target = 5;
        let ans = 2;
        assert_eq!(Solution::search_insert(nums, target), ans);

        let nums = vec![1, 3, 5, 6];
        let target = 2;
        let ans = 1;
        assert_eq!(Solution::search_insert(nums, target), ans);

        let nums = vec![1, 3, 5, 6];
        let target = 7;
        let ans = 4;
        assert_eq!(Solution::search_insert(nums, target), ans);
    }
}
