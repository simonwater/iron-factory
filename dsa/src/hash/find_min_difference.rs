//! [539. 最小时间差](https://leetcode.cn/problems/minimum-time-difference/)
//!

pub struct Solution;

impl Solution {
    pub fn find_min_difference(time_points: Vec<String>) -> i32 {
        let n = time_points.len();
        if n > 1440 {
            return 0;
        }
        let mut times = time_points
            .into_iter()
            .map(|s| {
                let bytes = s.as_bytes();
                let h = ((bytes[0] - b'0') * 10 + (bytes[1] - b'0')) as i32;
                let m = ((bytes[3] - b'0') * 10 + (bytes[4] - b'0')) as i32;
                h * 60 + m
            })
            .collect::<Vec<i32>>();

        times.sort_unstable();
        let mut ans = i32::MAX;
        let wins = times.windows(2);
        for win in wins {
            ans = ans.min(win[1] - win[0]);
        }
        ans.min(1440 + times[0] - times[n - 1])
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
