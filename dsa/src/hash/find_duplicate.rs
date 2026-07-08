/// # [287. 寻找重复数](https://leetcode.cn/problems/find-the-duplicate-number/)
///
/// 长度为n + 1的数组内放入给定范围[1, n]的数字，找出重复数字，重复数字只有一个
/// [1,3,4,2,2] -> 2
/// 原地哈希法（假设允许改变数组）：
/// 模拟使用堆上分配HashSet的过程，只是把当前数组自己当成哈希表。
/// 尝试把所有元素放回索引等于自己的位置上，等价于往哈希表中插入元素，遇到重复的直接返回，等价于哈希表中已经有这个元素。

pub struct Solution;

impl Solution {
    pub fn find_duplicate(mut nums: Vec<i32>) -> i32 {
        loop {
            // nums中没有0，用0位作为中转
            let cur_num = nums[0];
            let next_idx = cur_num as usize;
            let next_num = nums[next_idx];
            if next_num == cur_num {
                return cur_num;
            }
            nums.swap(0, next_idx);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
