/// [658. 找到 K 个最接近的元素](https://leetcode.cn/problems/find-k-closest-elements/)
struct Solution;

use std::collections::BinaryHeap;

impl Solution {
    /// 二分查找：因为范围固定是k，查找到左边界left，右边界就是left + k - 1，让左边界和右边界的后一个位置right = left + k比较，用排除法来理解：
    ///   • 如果right处离x更近，那么left和left左边的肯定更远，就全部排除，left前进一位，继续二分
    ///   • 如果left处离x更近或者或者和right处一样，那么right和右边的肯定更远，排除
    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let k = k as usize;
        let mut lo = 0;
        let mut hi = arr.len() - k;
        while lo < hi {
            let mid = lo + ((hi - lo) >> 1);
            if (arr[mid] - x).abs() > (arr[mid + k] - x).abs() {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        arr[lo..lo + k].to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let arr = vec![1, 2, 3, 4, 5];
        let k = 4;
        let x = 3;
        let ans = vec![1, 2, 3, 4];
        assert_eq!(Solution::find_closest_elements(arr, k, x), ans);

        let arr = vec![1, 1, 2, 3, 4, 5];
        let k = 4;
        let x = -1;
        let ans = vec![1, 1, 2, 3];
        assert_eq!(Solution::find_closest_elements(arr, k, x), ans);

        // 存在重复元素，且左、右距离相等时目前二分法有bug
        let arr = vec![1, 1, 2, 2, 2, 2, 2, 3, 3];
        let k = 3;
        let x = 3;
        let ans = vec![2, 3, 3];
        assert_eq!(Solution::find_closest_elements(arr, k, x), ans);
    }
}
