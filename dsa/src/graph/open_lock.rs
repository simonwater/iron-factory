/// # [752. 打开转盘锁](https://leetcode.cn/problems/open-the-lock/)
///

pub struct Solution;

use std::collections::VecDeque;
impl Solution {
    //
    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        let target: usize = target.parse().unwrap();
        if target == 0 {
            return 0;
        }
        let mut visited = [false; 10000];
        for s in deadends {
            let num: usize = s.parse().unwrap();
            if num == 0 || num == target {
                return -1;
            }
            visited[num] = true;
        }

        let mut q = VecDeque::with_capacity(10000);
        q.push_back(([0u16; 4], 0));
        visited[0] = true;
        while let Some((mut node, step)) = q.pop_front() {
            for i in 0..4 {
                let old = node[i];
                let new_step = step + 1;
                node[i] = (old + 1) % 10;
                let val1 = (node[0] * 1000 + node[1] * 100 + node[2] * 10 + node[3]) as usize;
                if val1 == target {
                    return new_step;
                }
                if !visited[val1] {
                    q.push_back((node, new_step));
                    visited[val1] = true;
                }

                node[i] = (old + 9) % 10;
                let val2 = (node[0] * 1000 + node[1] * 100 + node[2] * 10 + node[3]) as usize;
                if val2 == target {
                    return new_step;
                }
                if !visited[val2] {
                    q.push_back((node, new_step));
                    visited[val2] = true;
                }

                node[i] = old; // 还原
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
