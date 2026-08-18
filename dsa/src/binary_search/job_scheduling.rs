//! [1235. 规划兼职工作](https://leetcode.cn/problems/maximum-profit-in-job-scheduling/)
//!
//! 动态规划 + 二分查找
//!
//! dp[i] 表示 job[0..=i] 能获得的最大报酬
pub struct Solution;

impl Solution {
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        let n = start_time.len();
        let mut jobs = Vec::with_capacity(n);
        for i in 0..n {
            jobs.push((start_time[i], end_time[i], profit[i]));
        }
        jobs.sort_unstable_by_key(|k| k.1); // 按结束时间排序
        let mut dp = vec![0; n];
        dp[0] = jobs[0].2;
        for (i, &(start, _, prof)) in jobs.iter().enumerate().skip(1) {
            let idx = Self::find_index(&jobs[0..i], start);
            let prev = if idx == -1 { 0 } else { dp[idx as usize] };
            dp[i] = dp[i - 1].max(prev + prof);
        }
        dp[n - 1]
    }

    /// 在jobs中查找结束时间小于等于 start 的 **最后一个** 索引 (即满足条件的值的上界)
    /// 采用闭区间更新答案法，找到满足条件 **小于等于start** 的值后更新答案，然后继续尝试向右找。
    fn find_index(jobs: &[(i32, i32, i32)], start: i32) -> i32 {
        let n = jobs.len();
        if n == 0 {
            return -1;
        }
        let mut lo = 0;
        let mut hi = n - 1;
        let mut ans = -1;
        while lo <= hi {
            let mid = lo + ((hi - lo) >> 1);
            if jobs[mid].1 <= start {
                ans = mid as i32;
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
}

#[cfg(test)]
mod tests {
    use std::println;

    #[test]
    fn test1() {
        let vec = [0, 1, 2, 3];
        for (i, &num) in vec.iter().enumerate().skip(1) {
            println!("{}", i);
        }
    }
}
