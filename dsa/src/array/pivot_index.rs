/// # [724. 寻找数组的中心下标](https://leetcode.cn/problems/find-pivot-index/)
///

pub struct Solution;

// 前缀和
impl Solution {
    //
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut last_sum = vec![0; n];
        for i in (0..n - 1).rev() {
            last_sum[i] = nums[i + 1] + last_sum[i + 1];
        }
        let mut sum = 0;
        for (i, &num) in nums.iter().enumerate() {
            if sum == last_sum[i] {
                return i as i32;
            }
            sum += num;
        }

        -1
    }
}

// 空间优化
pub struct Solution2;

impl Solution2 {
    //
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let mut total = 0;
        for &num in &nums {
            total += num;
        }
        let mut prev = 0;
        for (i, &num) in nums.iter().enumerate() {
            if prev * 2 == total - prev - num {
                return i as i32;
            }
            prev += num;
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
