/// # [69. x 的平方根 ](https://leetcode.cn/problems/sqrtx/)
///

pub struct Solution;

impl Solution {
    //
    pub fn my_sqrt(x: i32) -> i32 {
        let x = x as i64;
        let mut hi = x;
        let mut lo = 0;
        let mut ans = 0;
        while lo <= hi {
            let guess = lo + ((hi - lo) >> 1);
            let val = guess * guess;
            if val == x {
                return guess as i32;
            } else if val > x {
                hi = guess - 1;
            } else {
                ans = guess;
                lo = guess + 1;
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
