/// [264. 丑数 II](https://leetcode.cn/problems/ugly-number-ii/)
pub struct Solution;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        let n = n as usize;
        let mut set = HashSet::with_capacity(n * 3);
        let mut heap = BinaryHeap::with_capacity(n * 3);
        set.insert(1i64);
        heap.push(Reverse(1i64));
        let mut ans = 1;
        for _ in 0..n {
            ans = heap.pop().unwrap().0;
            for p in [2, 3, 5] {
                let new_val = ans * p;
                if set.insert(new_val) {
                    heap.push(Reverse(new_val));
                }
            }
        }

        ans as i32
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
