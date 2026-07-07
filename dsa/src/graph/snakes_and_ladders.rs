/// # [909. 蛇梯棋](https://leetcode.cn/problems/snakes-and-ladders/)
/// bfs广度优先搜索

pub struct Solution;

use std::collections::VecDeque;
impl Solution {
    pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
        let n = board.len();
        let end = n * n;
        let mut visited = vec![false; end + 1];
        visited[1] = true;
        let mut q = VecDeque::with_capacity(end);
        q.push_back((1usize, 0)); // (num, step)
        while let Some((num, step)) = q.pop_front() {
            let new_step = step + 1;
            for i in 1..=6 {
                let mut next_num = num + i;
                if next_num > end {
                    break; // 超出的剪枝
                }
                let (r, c) = Self::get_pos(next_num, n);
                let val = board[r][c];
                // 传送一次
                if val != -1 {
                    next_num = val as usize;
                }

                if next_num == end {
                    return new_step;
                }

                if !visited[next_num] {
                    visited[next_num] = true;
                    q.push_back((next_num, new_step));
                }
            }
        }
        -1
    }

    fn get_pos(num: usize, n: usize) -> (usize, usize) {
        let mut r = (num - 1) / n;
        let mut c = (num - 1) % n;
        if r % 2 == 1 {
            c = n - 1 - c;
        }
        r = n - r - 1; // 从下往上
        (r, c)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
