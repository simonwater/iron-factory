/// [231. 2 的幂](https://leetcode.cn/problems/power-of-two/)
pub struct Solution;

impl Solution {
    pub fn is_power_of_two(mut n: i32) -> bool {
        if n <= 0 {
            return false;
        }
        let mut cnt = 0;
        while n != 0 {
            if n & 1 == 1 {
                cnt += 1;
            }
            n = n >> 1;
        }

        cnt == 1
    }
}

pub struct Solution2;

impl Solution2 {
    pub fn is_power_of_two(n: i32) -> bool {
        n > 0 && n & (n - 1) == 0
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
