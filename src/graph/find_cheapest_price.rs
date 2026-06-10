/// [787. K 站中转内最便宜的航班](https://leetcode.cn/problems/cheapest-flights-within-k-stops/)
struct Solution;

impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let mut prices = vec![i32::MAX / 2; n as usize];
        prices[src as usize] = 0;
        // 严格限制一次中转只向外扩展一层，防止一次中转内完成了多次中转的工作
        let mut tmp = prices.clone();
        for _ in 0..=k {
            let mut is_change = false;
            for edge in &flights {
                let from = edge[0] as usize;
                let to = edge[1] as usize;
                let price = edge[2];
                let new_val = prices[from] + price;
                if new_val < tmp[to] {
                    tmp[to] = new_val;
                    is_change = true;
                }
            }
            if !is_change {
                break;
            }
            prices.clone_from_slice(&tmp);
        }

        let ans = prices[dst as usize];
        if ans == i32::MAX / 2 { -1 } else { ans }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
