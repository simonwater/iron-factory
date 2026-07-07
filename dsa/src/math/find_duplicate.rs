/// # [287. 寻找重复数](https://leetcode.cn/problems/find-the-duplicate-number/)
///

pub struct Solution;

impl Solution {
    // 环状链表寻找环入口
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut slow = 0;
        let mut fast = 0;
        loop {
            slow = nums[slow] as usize;
            fast = nums[nums[fast] as usize] as usize;
            if slow == fast {
                break;
            }
        }
        let mut p = 0;
        while p != slow {
            p = nums[p] as usize;
            slow = nums[slow] as usize;
        }
        p as i32
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
