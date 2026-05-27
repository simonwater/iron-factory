/// [435. 无重叠区间](https://leetcode.cn/problems/non-overlapping-intervals/)
struct Solution;

impl Solution {
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        if intervals.len() <= 1 {
            return 0;
        }
        intervals.sort_unstable_by(|a, b| a[1].cmp(&b[1]));
        let mut ans = 0;
        let mut right = intervals[0][1];
        for cur in intervals.iter().skip(1) {
            if cur[0] < right {
                ans += 1;
            } else {
                right = cur[1];
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
