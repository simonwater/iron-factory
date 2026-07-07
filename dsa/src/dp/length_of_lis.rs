/// # [300. 最长递增子序列](https://leetcode.cn/problems/longest-increasing-subsequence/)
///

/// 动态规划：dp[i]表示以nums[i]结尾的最长递增子序列长度，中间记录全局最大长度值
pub struct Solution;

impl Solution {
    //
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dp = vec![1; n];
        let mut ans = 1;
        for i in 1..n {
            let cur = nums[i];
            for j in 0..i {
                if nums[j] < cur {
                    dp[i] = dp[i].max(dp[j] + 1);
                }
            }
            ans = ans.max(dp[i]);
        }
        ans
    }
}

pub struct Solution2;

impl Solution2 {
    //
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut tails: Vec<i32> = Vec::with_capacity(n);
        for num in nums {
            let i = Self::search_lower_bound(&tails, num);
            if i == tails.len() {
                tails.push(num);
            } else {
                tails[i] = num;
            }
        }

        tails.len() as i32
    }

    // 查找大于等于x的第一个位置
    fn search_lower_bound(nums: &[i32], x: i32) -> usize {
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
                if hi == 0 {
                    break;
                }
                hi -= 1;
            } else {
                lo += 1;
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
