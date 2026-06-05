/// [739. 每日温度](https://leetcode.cn/problems/daily-temperatures/)
struct Solution;

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let n = temperatures.len();
        let mut stack: Vec<(usize, i32)> = Vec::with_capacity(n);
        let mut ans = vec![0; n];
        for (idx, &t) in temperatures.iter().enumerate() {
            while let Some(&(prev_idx, prev_t)) = stack.last() {
                if prev_t < t {
                    ans[prev_idx] = (idx - prev_idx) as i32;
                    stack.pop();
                } else {
                    break;
                }
            }
            stack.push((idx, t));
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
