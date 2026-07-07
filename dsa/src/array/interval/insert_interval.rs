/// [57. 插入区间](https://leetcode.cn/problems/insert-interval/description/)
pub struct Solution;

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, mut new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = Vec::with_capacity(intervals.len() + 1);
        for interval in intervals {
            if interval[1] < new_interval[0] {
                ans.push(interval);
            } else if new_interval[1] < interval[0] {
                ans.push(new_interval.clone());
                ans.push(interval);
                new_interval[0] = i32::MAX;
            } else {
                new_interval[0] = i32::min(interval[0], new_interval[0]);
                new_interval[1] = i32::max(interval[1], new_interval[1]);
            }
        }
        if new_interval[0] != i32::MAX {
            ans.push(new_interval);
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let intervals = vec![vec![1, 3], vec![6, 9]];
        let new_interval = vec![2, 5];
        let ans = vec![vec![1, 5], vec![6, 9]];
        assert_eq!(Solution::insert(intervals, new_interval), ans);

        let intervals = vec![vec![1, 3], vec![6, 9]];
        let new_interval = vec![2, 5];
        let ans = vec![vec![1, 5], vec![6, 9]];
        assert_eq!(Solution::insert(intervals, new_interval), ans);
    }
}
