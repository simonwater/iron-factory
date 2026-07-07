/// # [377. 组合总和 Ⅳ](https://leetcode.cn/problems/combination-sum-iv/)
/// 求构成target的排列总数, 关键条件：  
/// 1. (1,2) (2,1)算不同的组合（相当于排列）
/// 2. 没有重复数字
/// 3. 同一数字可无限重复选择（完全背包）
///
/// 状态定义：`dp[i]` 表示总和为 i 的元素组合（排列）个数。
///
/// “外层跑金额，内层跑数字”：站在金额 i 的槽位上，横向让所有的数字全部过来汇报。
/// 代表我们允许任何一个数字（比如 num）作为这组排列的“最后一个收尾数字”

pub struct Solution;

impl Solution {
    // [70. 爬楼梯] 问题的扩展，爬楼梯相当于nums是[1, 2]
    pub fn combination_sum4(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort_unstable();
        let target = target as usize;
        let mut dp = vec![0; target + 1];
        dp[0] = 1; // 方便后续计算
        for i in 1..=target {
            for &num in &nums {
                let nums_usize = num as usize;
                if nums_usize > i {
                    break;
                }
                dp[i] += dp[i - nums_usize];
            }
        }
        dp[target]
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
