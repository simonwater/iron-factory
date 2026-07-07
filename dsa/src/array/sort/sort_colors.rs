/// [75. 颜色分类](https://leetcode.cn/problems/sort-colors/description/)
pub struct Solution;

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        if nums.len() <= 1 {
            return;
        }
        let (mut left, mut i, mut right) = (0, 0, nums.len() - 1);
        while i <= right {
            let cur = nums[i];
            if cur == 0 {
                nums.swap(left, i);
                left += 1;
                i += 1;
            } else if cur == 1 {
                i += 1;
            } else {
                nums.swap(i, right);
                if right == 0 {
                    break;
                }
                right -= 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
