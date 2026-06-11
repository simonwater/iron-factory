/// [785. 判断二分图](https://leetcode.cn/problems/is-graph-bipartite/)
struct Solution;

use std::collections::VecDeque;
impl Solution {
    // bfs 染色的同时判断是否冲突
    pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
        let n = graph.len();
        let mut states = vec![0; n];
        let mut q = VecDeque::with_capacity(n);
        for i in 0..n {
            // 跳过已经处理的，针对不连通的图
            if states[i] != 0 {
                continue;
            }

            states[i] = 1;
            q.push_back(i);
            while let Some(cur_node) = q.pop_front() {
                let cur_state = states[cur_node];
                for &next in &graph[cur_node] {
                    let next_node = next as usize;
                    let next_state = states[next_node];
                    if next_state == 0 {
                        states[next_node] = -cur_state; // 未访问到节点染成相反色。
                        q.push_back(next_node);
                    } else if next_state == cur_state {
                        return false;
                    }
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
