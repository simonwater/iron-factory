//! [274. H 指数](https://leetcode.cn/problems/h-index/)
//!

pub struct Solution;

impl Solution {
    pub fn h_index(mut citations: Vec<i32>) -> i32 {
        let mut ans = 0;
        citations.sort_unstable();
        for &num in citations.iter().rev() {
            if ans + 1 <= num {
                ans += 1;
            } else {
                break;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
