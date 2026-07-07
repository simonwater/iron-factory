/// [210. 课程表 II](https://leetcode.cn/problems/course-schedule-ii/)
pub struct Solution;
use std::collections::VecDeque;

/// bfs拓扑排序
impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let n = num_courses as usize;
        let mut in_degs = vec![0; n];
        let mut g: Vec<Vec<usize>> = vec![Vec::with_capacity(8); n];
        for e in prerequisites {
            let u = e[1] as usize;
            let v = e[0] as usize;
            g[u].push(v);
            in_degs[v] += 1;
        }

        let mut q = VecDeque::with_capacity(n);
        for (node, &in_deg) in in_degs.iter().enumerate() {
            if in_deg == 0 {
                q.push_back(node);
            }
        }

        // 由于长度不会超过n，所以可以直接用不同动态数组替代
        let mut ans = Vec::with_capacity(n);
        while let Some(node) = q.pop_front() {
            ans.push(node as i32);
            for &next in &g[node] {
                in_degs[next] -= 1;
                if in_degs[next] == 0 {
                    q.push_back(next);
                }
            }
        }

        if ans.len() == n { ans } else { vec![] }
    }
}

// dfs，前向时判断是否有环，回溯时收集路径节点
pub struct Solution2;
#[derive(PartialEq, Eq, Clone, Copy)]
enum State {
    UnVisited,
    Visiting,
    Visited,
}
impl Solution2 {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let n = num_courses as usize;
        let mut g = vec![Vec::with_capacity(8); n];
        let mut states = vec![State::UnVisited; n];
        let mut ans = Vec::with_capacity(n);
        for e in prerequisites {
            let u = e[1] as usize;
            let v = e[0] as usize;
            g[u].push(v);
        }
        for i in 0..n {
            if states[i] == State::UnVisited {
                if !Self::dfs(i, &g, &mut states, &mut ans) {
                    return vec![];
                }
            }
        }
        ans.reverse();
        ans
    }

    fn dfs(node: usize, g: &[Vec<usize>], states: &mut [State], ans: &mut Vec<i32>) -> bool {
        match states[node] {
            State::Visiting => false,
            State::Visited => true,
            _ => {
                states[node] = State::Visiting;
                for &next in &g[node] {
                    if !Self::dfs(next, g, states, ans) {
                        return false;
                    }
                }
                states[node] = State::Visited;
                ans.push(node as i32);
                true
            }
        }
    }
}

/// 链式前向星存储图
pub struct Solution3;
#[derive(Clone, Copy, Eq, PartialEq)]
struct Edge {
    pub to: usize,
    pub next: usize,
}
const EMPTY_EDGE: usize = usize::MAX;

impl Solution3 {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let n = num_courses as usize;
        let mut ans = Vec::with_capacity(n);
        let mut heads = vec![EMPTY_EDGE; n];
        let mut in_degs = vec![0; n];
        let mut edges: Vec<Edge> = Vec::with_capacity(prerequisites.len());
        for e in prerequisites {
            let u = e[1] as usize; // from
            let v = e[0] as usize; // to
            let edge = Edge {
                to: v,
                next: heads[u], // 指向现在的头节点
            };
            let e_idx = edges.len();
            edges.push(edge);
            heads[u] = e_idx; // 更新头节点
            in_degs[v] += 1;
        }
        let mut q: Vec<usize> = Vec::with_capacity(n); // 数组模拟队列
        for (node, &in_deg) in in_degs.iter().enumerate() {
            if in_deg == 0 {
                q.push(node);
            }
        }
        let mut q_head = 0;
        while q_head < q.len() {
            let cur_node = q[q_head];
            ans.push(cur_node as i32);
            q_head += 1;
            let mut e_idx = heads[cur_node]; // 从cur_node出发的第一条边的索引
            while e_idx != EMPTY_EDGE {
                let edge = &edges[e_idx];
                in_degs[edge.to] -= 1;
                if in_degs[edge.to] == 0 {
                    q.push(edge.to);
                }
                e_idx = edge.next; // 滑向下一条边
            }
        }
        if ans.len() == n { ans } else { vec![] }
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
