/// [300. 最长递增子序列](https://leetcode.cn/problems/longest-increasing-subsequence/)
pub struct Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut tails = Vec::with_capacity(n);
        for num in nums {
            let pos = Self::lower_bound(&tails, num);
            if pos == tails.len() {
                tails.push(num);
            } else {
                tails[pos] = num;
            }
        }
        tails.len() as i32
    }

    fn lower_bound(nums: &[i32], x: i32) -> usize {
        if nums.is_empty() {
            return 0;
        }
        let mut lo = 0;
        let mut hi = nums.len() - 1;
        let mut ans = nums.len();
        while lo <= hi {
            let mid = lo + ((hi - lo) >> 1);
            if nums[mid] >= x {
                ans = mid;
                if mid == 0 {
                    break;
                }
                hi = mid - 1;
            } else {
                lo = mid + 1;
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
