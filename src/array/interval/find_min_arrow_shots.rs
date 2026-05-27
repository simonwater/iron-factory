/// [452. 用最少数量的箭引爆气球](https://leetcode.cn/problems/minimum-number-of-arrows-to-burst-balloons/)
struct Solution;

impl Solution {
    // 左边界排序
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        if points.len() <= 1 {
            return points.len() as i32;
        }
        points.sort_unstable_by(|a, b| a[0].cmp(&b[0]));
        let mut iter = points.into_iter();
        let mut prev = iter.next().unwrap(); // 交集
        let mut ans = 1;
        for cur in iter {
            if cur[0] > prev[1] {
                prev = cur;
                ans += 1;
            } else {
                prev[0] = prev[0].max(cur[0]);
                prev[1] = prev[1].min(cur[1]);
            }
        }
        ans
    }
}

struct Solution2;

impl Solution2 {
    // 右边界排序
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        if points.len() <= 1 {
            return points.len() as i32;
        }
        points.sort_unstable_by(|a, b| a[1].cmp(&b[1]));
        let mut right = points[0][1];
        let mut ans = 1;
        for interval in points.iter().skip(1) {
            if interval[0] > right {
                right = interval[1];
                ans += 1;
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
