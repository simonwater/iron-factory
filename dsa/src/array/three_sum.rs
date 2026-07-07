/// [15. 三数之和](https://leetcode.cn/problems/3sum/description/)
pub struct Solution;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = vec![];
        if nums.len() <= 2 {
            return ans;
        }
        nums.sort_unstable();
        let n = nums.len();
        for i in 0..n - 2 {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            if nums[i] + nums[n - 1] + nums[n - 2] < 0 {
                continue;
            }
            if nums[i] + nums[i + 1] + nums[i + 2] > 0 {
                break;
            }
            let mut j = i + 1;
            let mut k = n - 1;
            while j < k {
                let sum = nums[i] + nums[j] + nums[k];
                if sum < 0 {
                    j += 1;
                } else if sum > 0 {
                    k -= 1;
                } else {
                    ans.push(vec![nums[i], nums[j], nums[k]]);
                    j += 1;
                    k -= 1;
                    while j < k && nums[j] == nums[j - 1] {
                        j += 1;
                    }
                    while j < k && nums[k] == nums[k + 1] {
                        k -= 1;
                    }
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![-1, 0, 1, 2, -1, -4];
        let ans = vec![[-1, -1, 2], [-1, 0, 1]];
        assert_eq!(Solution::three_sum(nums), ans);

        let nums = vec![0, 1, 1];
        let ans: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution::three_sum(nums), ans);

        let nums = vec![0, 0, 0];
        let ans = vec![[0, 0, 0]];
        assert_eq!(Solution::three_sum(nums), ans);
    }
}
