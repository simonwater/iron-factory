/// [658. 找到 K 个最接近的元素](https://leetcode.cn/problems/find-k-closest-elements/)
struct Solution;

use std::collections::BinaryHeap;

impl Solution {
    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let k = k as usize;
        let mut heap = BinaryHeap::with_capacity(k);
        for num in arr {
            let item = ((num - x).abs(), num);
            if heap.len() < k {
                heap.push(item);
            } else {
                if let Some(mut top) = heap.peek_mut() {
                    if item < *top {
                        *top = item;
                    }
                }
            }
        }
        let mut ans: Vec<i32> = heap.iter().map(|&(_, num)| num).collect();
        ans.sort_unstable();
        ans
    }

    pub fn find_closest_elements2(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        if arr.is_empty() || k == 0 {
            unreachable!();
        }
        let k = k as usize;
        let mut lo = 0;
        let mut hi = arr.len() - 1;
        while hi - lo + 1 > k {
            if (arr[hi] - x).abs() >= (arr[lo] - x).abs() {
                hi -= 1;
            } else {
                lo += 1;
            }
        }
        arr[lo..hi + 1].to_vec()
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
    }

    #[test]
    fn test2() {
        let arr = vec![1, 2, 3, 4, 5];
        let k = 4;
        let x = 3;
        let ans = vec![1, 2, 3, 4];
        assert_eq!(Solution::find_closest_elements2(arr, k, x), ans);

        let arr = vec![1, 1, 2, 3, 4, 5];
        let k = 4;
        let x = -1;
        let ans = vec![1, 1, 2, 3];
        assert_eq!(Solution::find_closest_elements2(arr, k, x), ans);
    }
}
