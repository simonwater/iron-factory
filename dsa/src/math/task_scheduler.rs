/// [621. 任务调度器](https://leetcode.cn/problems/task-scheduler/)
pub struct Solution;

impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        if n <= 0 {
            return tasks.len() as i32;
        }

        let mut counter = [0; 26];
        for &t in &tasks {
            let idx = (t as u8 - b'A') as usize;
            counter[idx] += 1;
        }
        let &max = counter.iter().max().unwrap_or(&0);
        let max_cnt = counter.iter().filter(|&&v| v == max).count() as i32;
        let intervals = (n + 1) * (max - 1) + max_cnt;
        intervals.max(tasks.len() as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let tasks = vec!['A', 'A', 'A', 'B', 'B', 'B'];
        let n = 2;
        let ans = 8;
        assert_eq!(Solution::least_interval(tasks, n), ans);

        let tasks = vec!['A', 'C', 'A', 'B', 'D', 'B'];
        let n = 1;
        let ans = 6;
        assert_eq!(Solution::least_interval(tasks, n), ans);

        let tasks = vec!['A', 'A', 'A', 'B', 'B', 'B'];
        let n = 3;
        let ans = 10;
        assert_eq!(Solution::least_interval(tasks, n), ans);
    }
}
