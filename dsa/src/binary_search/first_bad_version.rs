/// [278. 第一个错误的版本](https://leetcode.cn/problems/first-bad-version/)
pub struct Solution;

impl Solution {
    pub fn first_bad_version(&self, n: i32) -> i32 {
        let mut lo = 1;
        let mut hi = n;
        let mut ans = n;
        while lo <= hi {
            let mid = lo + ((hi - lo) >> 1);
            if self.is_bad_version(mid) {
                ans = mid;
                hi = mid - 1;
            } else {
                lo = mid + 1;
            }
        }
        ans
    }

    fn is_bad_version(&self, _version: i32) -> bool {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
