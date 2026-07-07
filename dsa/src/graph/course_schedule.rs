/// [207. 课程表](https://leetcode.cn/problems/course-schedule/)
pub struct Solution;
use std::collections::VecDeque;

/// 拓扑排序
impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let num_courses = num_courses as usize;
        let mut g: Vec<Vec<usize>> = vec![Vec::with_capacity(10); num_courses];
        let mut in_deg: Vec<usize> = vec![0; num_courses];
        for e in prerequisites {
            let u = e[1] as usize;
            let v = e[0] as usize;
            g[u].push(v);
            in_deg[v] += 1;
        }

        let mut q = VecDeque::new();
        for (node, &deg) in in_deg.iter().enumerate() {
            if deg == 0 {
                q.push_back(node);
            }
        }
        let mut out_cnt = 0;
        while let Some(node) = q.pop_front() {
            out_cnt += 1;
            for &next in &g[node] {
                in_deg[next] -= 1;
                if in_deg[next] == 0 {
                    q.push_back(next);
                }
            }
        }
        out_cnt == num_courses
    }
}

pub struct Solution2;

impl Solution2 {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let num_courses = num_courses as usize;
        let mut g: Vec<Vec<usize>> = vec![Vec::with_capacity(10); num_courses];
        for e in prerequisites {
            let u = e[1] as usize;
            let v = e[0] as usize;
            g[u].push(v);
        }
        // 0: 未访问，1:访问中，2:已访问
        let mut states = vec![0; num_courses];
        for node in 0..num_courses {
            if states[node] == 0 {
                if !Self::dfs(node, &g, &mut states) {
                    return false;
                }
            }
        }
        true
    }

    fn dfs(node: usize, g: &Vec<Vec<usize>>, states: &mut [i32]) -> bool {
        match states[node] {
            // 访问中
            1 => false,
            // 已访问
            2 => true,
            // 未访问
            _ => {
                states[node] = 1;
                for &next in &g[node] {
                    if !Self::dfs(next, g, states) {
                        return false;
                    }
                }
                states[node] = 2;
                true
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let edges = vec![vec![1, 0], vec![1, 2], vec![0, 1]];
        let ans = false;
        assert_eq!(Solution::can_finish(3, edges), ans);
    }
}
