/// [70. 爬楼梯](https://leetcode.cn/problems/climbing-stairs/)
struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let (mut a, mut b) = (1, 1); // （0阶, 1阶)
        for _ in 2..=n {
            (a, b) = (b, a + b);
        }
        b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
