/// [134. 加油站](https://leetcode.cn/problems/gas-station/)
pub struct Solution;

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut amount = 0;
        let mut min_gas = i32::MAX;
        let mut ans = 0;
        for i in 0..gas.len() {
            // 计算的是到达下一站时剩余的油量
            amount += gas[i] - cost[i];
            if amount < min_gas {
                min_gas = amount;
                ans = i + 1;
            }
        }
        if amount < 0 {
            -1
        } else {
            (ans % gas.len()) as i32
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
