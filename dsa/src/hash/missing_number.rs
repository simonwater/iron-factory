use std::collections::HashSet;

/// # [268. 丢失的数字](https://leetcode.cn/problems/missing-number/)
///

pub struct Solution;
/// nums = [9,6,4,2,3,5,7,0,1] ans = 8
impl Solution {
    // 简单哈希表
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let set: HashSet<i32> = HashSet::from_iter(nums);
        for num in 0..=n {
            if !set.contains(&num) {
                return num;
            }
        }
        -1
    }
}

/// 根据数据取值范围，原地哈希把数组自己当作哈希表来使用
pub struct Solution2;
/// nums = [9,6,4,2,3,5,7,0,1] ans = 8
impl Solution2 {
    pub fn missing_number(mut nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        // 第1步类似于把数字插入哈希表，原地哈希表现为归位到索引和值相同的位置
        for idx in 0..nums.len() {
            while nums[idx] != n && nums[idx] != idx as i32 {
                let cur = nums[idx];
                let next_idx = cur as usize;
                let next = nums[next_idx];
                if next != cur {
                    nums.swap(idx, next_idx);
                }
            }
        }
        // 第2步类似于挨个数字判断哈希表中是否已经有该数字
        for (i, &num) in nums.iter().enumerate() {
            let i = i as i32;
            if i != num {
                return i;
            }
        }
        n
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
