/// [743. 网络延迟时间](https://leetcode.cn/problems/network-delay-time/)
/// dijkstra算法
pub struct Solution;
use std::{cmp::Reverse, collections::BinaryHeap};
impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let n = n as usize;
        let start = k as usize;
        let mut g: Vec<Vec<(usize, i32)>> = vec![Vec::with_capacity(8); n + 1]; // 邻接表
        for e in times {
            let u = e[0] as usize;
            let v = e[1] as usize;
            let t = e[2];
            g[u].push((v, t));
        }

        let mut states = vec![i32::MAX; n + 1];
        states[0] = 0;
        states[start] = 0;
        let mut heap = BinaryHeap::with_capacity(n);
        // 通过Reverse转为小顶堆：(当前距离, 节点)
        heap.push((Reverse(0), start));
        while let Some((Reverse(state), cur_node)) = heap.pop() {
            if state > states[cur_node] {
                // 说明对于当前节点，已经处理了更好的松弛结果，防止重复计算
                continue;
            }
            for &(next_node, time) in &g[cur_node] {
                let new_state = state + time;
                if new_state < states[next_node] {
                    states[next_node] = new_state;
                    heap.push((Reverse(new_state), next_node));
                }
            }
        }

        let mut ans = i32::MIN;
        for &t in &states {
            if t > ans {
                ans = t;
            }
            if ans == i32::MAX {
                return -1;
            }
        }
        ans
    }
}

// 链式前向星存储
struct Edge {
    to: usize,
    next: usize,
    time: i32,
}
const EMPTY_EDGE: usize = usize::MAX;
pub struct Solution2;
impl Solution2 {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let n = n as usize;
        let start = k as usize;
        let mut heads = vec![usize::MAX; n + 1];
        let mut edges: Vec<Edge> = Vec::with_capacity(times.len());
        for e in times {
            let u = e[0] as usize;
            let v = e[1] as usize;
            let t = e[2];
            let edge = Edge {
                to: v,
                next: heads[u],
                time: t,
            };
            heads[u] = edges.len();
            edges.push(edge);
        }

        let mut states = vec![i32::MAX; n + 1];
        states[0] = 0;
        states[start] = 0;
        let mut heap = BinaryHeap::with_capacity(n);
        // 通过Reverse转为小顶堆：(当前距离, 节点)
        heap.push((Reverse(0), start));
        while let Some((Reverse(state), cur_node)) = heap.pop() {
            if state > states[cur_node] {
                // 说明对于当前节点，已经处理了更好的松弛结果，防止重复计算
                continue;
            }
            let mut e_idx = heads[cur_node];
            while e_idx != EMPTY_EDGE {
                let e = &edges[e_idx];
                let (next_node, time) = (e.to, e.time);
                let new_state = state + time;
                if new_state < states[next_node] {
                    states[next_node] = new_state;
                    heap.push((Reverse(new_state), next_node));
                }
                e_idx = e.next;
            }
        }

        let mut ans = i32::MIN;
        for &t in &states {
            if t > ans {
                ans = t;
            }
            if ans == i32::MAX {
                return -1;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
