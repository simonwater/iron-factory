/// # [713. 乘积小于 K 的子数组](https://leetcode.cn/problems/subarray-product-less-than-k/)
///

pub struct Solution;

impl Solution {
    // 滑动窗口
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        if k <= 1 || nums.is_empty() {
            return 0;
        }
        let mut ans = 0;
        let mut left = 0;
        let mut p = 1;
        for (i, &num) in nums.iter().enumerate() {
            if num >= k {
                left = i + 1;
                p = 1;
                continue;
            }

            p *= num;
            while p >= k {
                p /= nums[left];
                left += 1;
            }
            ans += (i - left + 1) as i32;
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
