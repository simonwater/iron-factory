/// [70. 爬楼梯](https://leetcode.cn/problems/climbing-stairs/)
pub struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let (mut a, mut b) = (1, 1); // （0阶, 1阶)
        for _ in 2..=n {
            (a, b) = (b, a + b);
        }
        b
    }
}

/// [746. 使用最小花费爬楼梯](https://leetcode.cn/problems/min-cost-climbing-stairs/)
/// 可以选择从下标为 0 或下标为 1 的台阶开始，所以爬上前两个台阶的代价都是0
/// 从爬上第三个台阶的最小代价开始计算。根据题意，需要爬到cost.len()的位置。
impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let (mut a, mut b) = (0, 0);
        for win in cost.windows(2) {
            // 据说用windows返回的切片编译器编译时会跳过边界检查？
            let cur = (a + win[0]).min(b + win[1]);
            (a, b) = (b, cur);
        }
        b
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
