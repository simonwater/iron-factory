/// [189. 轮转数组](https://leetcode.cn/problems/rotate-array/)
struct Solution;

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, mut k: i32) {
        let mut k = k as usize;
        let n = nums.len();
        if n <= 1 || k % n == 0 {
            return;
        }
        k = k % n;
        Self::reverse(nums, 0, n - 1);
        Self::reverse(nums, 0, k - 1);
        Self::reverse(nums, k as usize, n - 1);
    }

    fn reverse(nums: &mut [i32], mut start: usize, mut end: usize) {
        while start < end {
            nums.swap(start, end);
            start += 1;
            end -= 1;
        }
    }
}

struct Solution2;

impl Solution2 {
    pub fn rotate(nums: &mut Vec<i32>, mut k: i32) {
        let mut k = k as usize;
        let n = nums.len();
        if n <= 1 || k % n == 0 {
            return;
        }
        k = k % n;
        nums.reverse();
        nums[..k].reverse();
        nums[k..].reverse();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
