/// # [55. 跳跃游戏](https://leetcode.cn/problems/jump-game/)
/// 是否能够跳到最后一个位置

pub struct Solution;

impl Solution {
    //
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut furthest = 0;
        for (i, &num) in nums.iter().enumerate() {
            if furthest < i {
                return false;
            }
            furthest = furthest.max(i + (num as usize));
            if furthest >= nums.len() - 1 {
                return true;
            }
        }
        true
    }
}

/// # [45. 跳跃游戏 II](https://leetcode.cn/problems/jump-game-ii/description/)
/// 跳到最后一个位置的最小步数
pub struct SolutionII;

impl SolutionII {
    // 遍历所有的起跳点（最后一个位置必然不是起跳点）
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut cur_end = 0;
        let mut next = 0;
        let mut ans = 0;
        for i in 0..nums.len() - 1 {
            let num = nums[i] as usize;
            next = next.max(i + num);
            if i == cur_end {
                ans += 1;
                cur_end = next;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
