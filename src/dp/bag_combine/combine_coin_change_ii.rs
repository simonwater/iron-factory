/// # [518. 零钱兑换 II](https://leetcode.cn/problems/coin-change-ii/)
/// 求组成target的 组合总数
/// 关键条件：
///     1. 没有重复硬币
///     2. 每种硬币数量无限，可无限重复选择（完全背包）

/// # 记忆化递归搜索，数字选与不选
/// 递归函数语义为: `dfs(i, amt)` 表示 `coins[0..i]` (前i个元素)中能够构成amt的组合总数。   
/// 根据 `coins[i - 1]` 选择或者不选择，能得到问题规模缩小的关系是：
/// `dfs(i, amt) = dfs(i, amt - coins[i - 1]) + dfs(i - 1, amt)`
struct MemoDFS1;

impl MemoDFS1 {
    pub fn change(amount: i32, mut coins: Vec<i32>) -> i32 {
        let amount = amount as usize;
        let mut memo = vec![vec![-1; coins.len() + 1]; amount + 1];
        Self::dfs(coins.len(), &coins, amount, &mut memo)
    }

    fn dfs(i: usize, nums: &[i32], amount: usize, memo: &mut Vec<Vec<i32>>) -> i32 {
        if amount == 0 {
            return 1;
        }
        if i == 0 {
            return 0;
        }
        if memo[amount][i] != -1 {
            return memo[amount][i];
        }
        let c = nums[i - 1] as usize;
        if amount < c {
            memo[amount][i] = Self::dfs(i - 1, nums, amount, memo);
        } else {
            memo[amount][i] =
                Self::dfs(i, nums, amount - c, memo) + Self::dfs(i - 1, nums, amount, memo);
        }
        memo[amount][i]
    }
}

/// # 自顶向下的递归翻译为自底向上的递推
/// 状态定义：
///
/// `dp[amt][i]` 表示通过`coins[0..i]`中的数据(前i个数)构成 amt 的组合总数。<br>
/// 所以`dp[amt][i] = dp[amt][i - 1] + dp[amt - coins[i - 1]][i]` (不选`coins[i - 1]` + 选择`coins[i - 1]`)
struct Solution2;

impl Solution2 {
    pub fn change(amount: i32, mut coins: Vec<i32>) -> i32 {
        let amount = amount as usize;
        let n = coins.len();
        let mut dp = vec![vec![0; n + 1]; amount + 1];
        // 为方便后续计算，amt为0的组合数设为1。看递推公式可知选择coins[i - 1]时数量是dp[amt - coins[i - 1]][i]
        // 硬币coins[i - 1]恰好等于amt时，成功得到了一个组合，所以数量上应该加1
        for val in dp[0].iter_mut() {
            *val = 1;
        }

        for amt in 1..=amount {
            for i in 1..=n {
                let c = coins[i - 1] as usize;
                if c > amt {
                    dp[amt][i] = dp[amt][i - 1];
                } else {
                    // 选择 + 不选择
                    dp[amt][i] += dp[amt - c][i] + dp[amt][i - 1];
                }
            }
        }
        dp[amount][n]
    }
}

/// # 递推+状态空间压缩
/// 按照“外层跑金额，内层跑硬币”的循环方式，一维的状态数组中会有重复结果。因为数字每次都从头枚举，会把排列当组合处理。
/// 以[1,2]构成3为例，以1结尾时：`dp[3] += dp[3 - 1]`,对应的组合是(1,1,1)和(2,1)， 枚举到2结尾时，`dp[3] += dp[3 - 2]`,
/// 对应的组合为 (1, 2)。所以构成3的所有组合是(1,1,1)，(2,1)和(1, 2)，`dp[3]`结果为错误的3。
///
struct Solution3;

impl Solution3 {
    /// dp[i],凑成金额i的组合总数。
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        0
    }
}

/// # 记忆化递归搜索，枚举所有可能
/// MemoDFS2性能更差，耗时是MemoDFS1的20倍。
struct MemoDFS2;

impl MemoDFS2 {
    pub fn change(amount: i32, mut coins: Vec<i32>) -> i32 {
        coins.sort_unstable();
        let mut memo = vec![vec![-1; coins.len()]; amount as usize + 1];
        Self::dfs(0, &coins, amount, &mut memo)
    }

    fn dfs(start: usize, nums: &[i32], amount: i32, memo: &mut Vec<Vec<i32>>) -> i32 {
        if amount == 0 {
            return 1;
        }
        if start == nums.len() {
            return 0;
        }
        if memo[amount as usize][start] != -1 {
            return memo[amount as usize][start];
        }

        let mut ans = 0;
        for i in start..nums.len() {
            if nums[i] > amount {
                break;
            }
            ans += Self::dfs(i, nums, amount - nums[i], memo);
        }

        memo[amount as usize][start] = ans;
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
