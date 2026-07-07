/// [16. 最接近的三数之和](https://leetcode.cn/problems/3sum-closest/description/)
pub struct Solution;

impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();
        if n <= 2 {
            panic!("unexpected argument: nums");
        }
        nums.sort_unstable();
        let mut ans = nums[0] + nums[1] + nums[2];
        for i in 0..n - 2 {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            let mut sum = nums[i] + nums[i + 1] + nums[i + 2];
            if sum > target {
                if (sum - target).abs() < (ans - target).abs() {
                    ans = sum;
                }
                break;
            }

            sum = nums[i] + nums[n - 2] + nums[n - 1];
            if sum < target {
                if (sum - target).abs() < (ans - target).abs() {
                    ans = sum;
                }
                continue;
            }

            let mut j = i + 1;
            let mut k = n - 1;
            while j < k {
                sum = nums[i] + nums[j] + nums[k];
                if sum == target {
                    return sum;
                }
                if (sum - target).abs() < (ans - target).abs() {
                    ans = sum;
                }

                if sum < target {
                    j += 1;
                } else {
                    k -= 1;
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
        let nums = vec![-1, 2, 1, -4];
        assert_eq!(Solution::three_sum_closest(nums, 1), 2);

        let nums = vec![0, 0, 0];
        assert_eq!(Solution::three_sum_closest(nums, 1), 0);
    }
}
