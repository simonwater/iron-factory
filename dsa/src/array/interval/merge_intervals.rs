/// [56. 合并区间](https://leetcode.cn/problems/merge-intervals/description/)
pub struct Solution;

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if intervals.len() == 0 {
            return Vec::new();
        }
        intervals.sort_unstable_by_key(|a| a[0]);
        let mut ans: Vec<Vec<i32>> = Vec::with_capacity(intervals.len());
        let mut iter = intervals.into_iter();
        let mut prev = iter.next().unwrap();
        for cur in iter {
            if prev[1] < cur[0] {
                ans.push(prev);
                prev = cur;
            } else {
                prev[1] = prev[1].max(cur[1]);
            }
        }
        ans.push(prev);
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let intervals = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];
        let ans = vec![vec![1, 6], vec![8, 10], vec![15, 18]];
        assert_eq!(Solution::merge(intervals), ans);

        let intervals = vec![vec![1, 4], vec![4, 5]];
        let ans = vec![vec![1, 5]];
        assert_eq!(Solution::merge(intervals), ans);

        let intervals = vec![vec![4, 7], vec![1, 4]];
        let ans = vec![vec![1, 7]];
        assert_eq!(Solution::merge(intervals), ans);
    }
}
