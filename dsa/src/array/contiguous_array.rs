/// [525. 连续数组](https://leetcode.cn/problems/contiguous-array/)
pub struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::with_capacity(nums.len());
        map.insert(0, -1);
        let mut ans = 0;
        let mut sum = 0;
        for (idx, &num) in nums.iter().enumerate() {
            sum += if num == 1 { 1 } else { -1 };
            let idx_i32 = idx as i32;
            ans = ans.max(idx_i32 - *map.entry(sum).or_insert(idx_i32));
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
