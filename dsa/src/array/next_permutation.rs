/// [31. 下一个排列](https://leetcode.cn/problems/next-permutation/)
/// 1. 从后往前，找到第一个变小的位置: [2, 5, 4, 3, 2, 1] 停在数字5处
/// 2. 从后往前，找到第一个比待交换数字(2)大的数。[2, 5, 4, 3, 2, 1]中找到3，然后交换
/// 3. 降序变升序，得到第一个比当前值大的数
pub struct Solution;
impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let n = nums.len();
        if n <= 1 {
            return;
        }
        // 1. 从后往前，找到第一个变小的位置: [2, 5, 4, 3, 2, 1] 停在数字5处
        let mut j = nums.len() - 1;
        while j > 0 && nums[j - 1] >= nums[j] {
            j -= 1;
        }

        if j > 0 {
            // 2. 从后往前，找到第一个比待交换数字(2)大的数。[2, 5, 4, 3, 2, 1]中找到3，然后交换
            let cur = nums[j - 1];
            let mut i = nums.len() - 1;
            while cur >= nums[i] {
                i -= 1;
            }
            nums.swap(j - 1, i);
        }

        // 3. 降序变升序，得到第一个比当前值大的数
        let mut k = nums.len() - 1;
        while j < k {
            nums.swap(j, k);
            j += 1;
            k -= 1;
        }
    }
}
