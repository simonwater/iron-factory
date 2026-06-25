/// # [279. 完全平方数](https://leetcode.cn/problems/perfect-squares/)
/// 等价于一组不同的数字，每个数字可重复选择，求能构成目标值的最少数字个数
/// 完全背包求组合，同322. 零钱兑换

struct Solution;

impl Solution {
    //
    pub fn num_squares(n: i32) -> i32 {
        let len = f64::sqrt(n as f64) as usize + 1;
        let mut nums = Vec::with_capacity(len);
        let n = n as usize;
        // 预先准备好备选数字
        for i in 1..=n {
            let num = i * i;
            if num <= n {
                nums.push(num);
            } else {
                break;
            }
        }
        let mut dp = vec![0; n as usize + 1];
        for v in 1..=n {
            dp[v] = v;
            for &num in &nums {
                if num > v {
                    break;
                }
                dp[v] = dp[v].min(dp[v - num] + 1)
            }
        }

        dp[n] as i32
    }
}

struct Solution2;

impl Solution2 {
    //
    pub fn num_squares(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![0; n + 1];
        for v in 1..=n {
            dp[v] = v;
            let mut num = 1;
            while num * num <= v {
                dp[v] = dp[v].min(dp[v - num * num] + 1);
                num += 1;
            }
        }

        dp[n] as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
