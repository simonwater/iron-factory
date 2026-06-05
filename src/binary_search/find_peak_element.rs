/// [162. 寻找峰值](https://leetcode.cn/problems/find-peak-element/)
struct Solution;

impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return 0;
        }
        let mut lo = 0;
        let mut hi = nums.len() - 1;
        while lo <= hi {
            let mid = lo + ((hi - lo) >> 1);
            let mid_val = nums[mid] as i64;
            let prev_val = if mid == 0 {
                i64::MIN
            } else {
                nums[mid - 1] as i64
            };
            let next_val = if mid == nums.len() - 1 {
                i64::MIN
            } else {
                nums[mid + 1] as i64
            };

            if prev_val < mid_val && mid_val > next_val {
                return mid as i32;
            } else if mid_val > next_val {
                hi = mid - 1;
            } else {
                lo = mid + 1;
            }
        }
        unreachable!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
