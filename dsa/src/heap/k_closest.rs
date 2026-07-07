/// [973. 最接近原点的 K 个点](https://leetcode.cn/problems/k-closest-points-to-origin/)
pub struct Solution;

use std::collections::BinaryHeap;
impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let k = k as usize;
        let mut heap = BinaryHeap::with_capacity(k);
        for p in points {
            let d = p[0] * p[0] + p[1] * p[1];
            if heap.len() < k {
                heap.push((d, p));
            } else if let Some(v) = heap.peek() {
                if d < v.0 {
                    heap.pop();
                    heap.push((d, p));
                }
            }
        }
        heap.into_iter().map(|(_, p)| p).collect()
    }

    pub fn k_closest2(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let k = k as usize;
        let mut heap = BinaryHeap::with_capacity(k + 1);
        for p in points {
            let d = p[0] * p[0] + p[1] * p[1];
            heap.push((d, p));
            if heap.len() > k {
                heap.pop();
            }
        }
        heap.into_iter().map(|(_, p)| p).collect()
    }

    pub fn k_closest3(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let k = k as usize;
        let mut heap = BinaryHeap::with_capacity(k);
        for p in points {
            let d = p[0] * p[0] + p[1] * p[1];
            if heap.len() < k {
                heap.push((d, p));
            } else if let Some(mut v) = heap.peek_mut() {
                if d < v.0 {
                    *v = (d, p);
                }
            }
        }
        heap.into_iter().map(|(_, p)| p).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let points = vec![vec![1, 3], vec![-2, 2]];
        let k = 1;
        let ans = vec![vec![-2, 2]];
        assert_eq!(Solution::k_closest(points, k), ans);

        let points = vec![vec![3, 3], vec![5, -1], vec![-2, 4]];
        let k = 2;
        let ans = vec![vec![-2, 4], vec![3, 3]];
        assert_eq!(Solution::k_closest(points, k), ans);
    }

    #[test]
    fn test2() {
        let points = vec![vec![1, 3], vec![-2, 2]];
        let k = 1;
        let ans = vec![vec![-2, 2]];
        assert_eq!(Solution::k_closest2(points, k), ans);

        let points = vec![vec![3, 3], vec![5, -1], vec![-2, 4]];
        let k = 2;
        let ans = vec![vec![-2, 4], vec![3, 3]];
        assert_eq!(Solution::k_closest2(points, k), ans);
    }

    #[test]
    fn test3() {
        let points = vec![vec![1, 3], vec![-2, 2]];
        let k = 1;
        let ans = vec![vec![-2, 2]];
        assert_eq!(Solution::k_closest3(points, k), ans);

        let points = vec![vec![3, 3], vec![5, -1], vec![-2, 4]];
        let k = 2;
        let ans = vec![vec![-2, 4], vec![3, 3]];
        assert_eq!(Solution::k_closest3(points, k), ans);
    }
}
