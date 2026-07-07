/// [528. 按权重随机选择](https://leetcode.cn/problems/random-pick-with-weight/)
use rand::Rng;
pub struct Solution {
    intervals: Vec<(i32, i32)>,
    total: i32,
}

impl Solution {
    pub fn new(w: Vec<i32>) -> Self {
        let mut intervals = Vec::with_capacity(w.len());
        let mut start = 0;
        for cnt in w {
            intervals.push((start, start + cnt - 1)); // 闭区间
            start = start + cnt;
        }
        Self {
            intervals,
            total: start,
        }
    }

    pub fn pick_index(&self) -> i32 {
        let mut rng = rand::rng();
        let x = rng.random_range(0..self.total);
        let mut lo = 0;
        let mut hi = self.intervals.len() - 1;
        while lo <= hi {
            let mid = lo + ((hi - lo) >> 1);
            let start = self.intervals[mid].0;
            let end = self.intervals[mid].1;
            if x >= start && x <= end {
                return mid as i32;
            } else if x < start {
                hi = mid - 1;
            } else {
                lo = mid + 1;
            }
        }
        unreachable!();
    }
}

pub struct Solution2 {
    bounds: Vec<i32>,
    total: i32,
}

impl Solution2 {
    pub fn new(w: Vec<i32>) -> Self {
        let mut bounds = Vec::with_capacity(w.len());
        let mut total = w[0];
        bounds.push(total - 1); // 右边界
        for &i in w.iter().skip(1) {
            total += i;
            bounds.push(total - 1);
        }
        Self { bounds, total }
    }

    pub fn pick_index(&self) -> i32 {
        let mut rng = rand::rng();
        let x = rng.random_range(0..self.total);
        // 寻找大于等于x的第一个索引
        let mut lo = 0;
        let mut hi = self.bounds.len() - 1;
        let mut ans = hi;
        while lo <= hi {
            let mid = lo + ((hi - lo) >> 1);
            let mid_val = self.bounds[mid];
            if mid_val >= x {
                ans = mid;
                if mid == 0 {
                    break;
                }
                hi = mid - 1;
            } else {
                lo = mid + 1;
            }
        }
        ans as i32
    }
}
