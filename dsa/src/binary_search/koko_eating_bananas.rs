/// # [875. 爱吃香蕉的珂珂](https://leetcode.cn/problems/koko-eating-bananas/)
///
pub struct Solution;

impl Solution {
    //
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let n = piles.len();
        let h_usize = h as usize;
        let h_i64 = h as i64;
        if n == 0 || h_usize < n {
            unreachable!();
        }
        let mut lo = 1;
        let mut hi = piles.iter().max().copied().unwrap();
        if h_usize == n {
            return hi;
        }
        let mut ans = 1;
        while lo <= hi {
            let mid = lo + ((hi - lo) >> 1); // 每小时吃的值
            let need_h = Self::need_hours(&piles, mid);
            if need_h <= h_i64 {
                // 吃完还有剩余时间，记录一个可能解，并降慢速度继续尝试
                ans = mid;
                hi = mid - 1;
            } else {
                // 速度太慢吃不完，加快速度
                lo = mid + 1;
            }
        }
        ans
    }

    // 每小时吃 k 根，需要多小时吃完
    fn need_hours(piles: &[i32], k: i32) -> i64 {
        let mut h = 0;
        for &pile in piles {
            h += ((pile + k - 1) / k) as i64;
        }
        h
    }

    // 防溢出写法
    fn _need_hours_(piles: &[i32], k: i32) -> i64 {
        let mut h = 0;
        for &pile in piles {
            h += (pile / k + if pile % k != 0 { 1 } else { 0 }) as i64;
        }
        h
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let piles = vec![805306368, 805306368, 805306368];
        let h = 1000000000;
        assert_eq!(3, Solution::min_eating_speed(piles, h));
    }
}
