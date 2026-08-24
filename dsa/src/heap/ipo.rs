//! [502. IPO](https://leetcode.cn/problems/ipo/)
//!

use std::collections::BinaryHeap;
pub struct Solution;

impl Solution {
    pub fn find_maximized_capital(
        mut k: i32,
        mut w: i32,
        profits: Vec<i32>,
        capital: Vec<i32>,
    ) -> i32 {
        let n = profits.len();
        let mut projects = Vec::with_capacity(n);
        for i in 0..n {
            projects.push((profits[i], capital[i]));
        }
        projects.sort_unstable_by_key(|v| v.1);
        let mut heap = BinaryHeap::with_capacity(n); // 堆里存放当前资本满足启动的项目
        for (prf, cap) in projects {
            // 发现当前项目已经无法吃下，则回过头先消灭能启动的项目
            while w < cap && k > 0 && !heap.is_empty() {
                let p = heap.pop().unwrap();
                w += p;
                k -= 1;
            }

            if w >= cap {
                // 已经能拿下当前项目，先攒起来
                heap.push(prf);
            }
            if k == 0 || heap.is_empty() {
                return w;
            }
        }
        while k > 0 && !heap.is_empty() {
            let p = heap.pop().unwrap();
            w += p;
            k -= 1;
        }
        w
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
