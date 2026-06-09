/// [310. 最小高度树](https://leetcode.cn/problems/minimum-height-trees/)
struct Solution;
use std::collections::VecDeque;
struct Edge {
    pub to: usize,
    pub next: usize,
}
impl Edge {
    pub fn new(to: usize, next: usize) -> Self {
        Self { to, next }
    }
}
const EMPTY_EDGE: usize = usize::MAX;

impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        if n == 1 {
            return vec![0];
        }
        let n = n as usize;
        let mut heads = vec![EMPTY_EDGE; n];
        let mut degs = vec![0; n];
        let mut es = Vec::with_capacity(edges.len() * 2);
        for e in edges {
            let u = e[0] as usize;
            let v = e[1] as usize;
            let e1 = Edge::new(v, heads[u]);
            heads[u] = es.len();
            es.push(e1);

            let e2 = Edge::new(u, heads[v]);
            heads[v] = es.len();
            es.push(e2);

            degs[u] += 1;
            degs[v] += 1;
        }

        let mut q: VecDeque<usize> = VecDeque::with_capacity(n);
        for (idx, &val) in degs.iter().enumerate() {
            if val == 1 {
                q.push_back(idx);
            }
        }
        // 表示剩余节点数。从度最小节点开始一层层往里剥，每剥完一层，发现剩余节点小于等于2则结束。
        let mut lefts = n;
        while lefts > 2 {
            let sz = q.len();
            lefts -= sz;
            for _ in 0..sz {
                let node = q.pop_front().unwrap();
                let mut next_edge = heads[node];
                while next_edge != EMPTY_EDGE {
                    let e = &es[next_edge];
                    let next_node = e.to;
                    degs[next_node] -= 1;
                    if degs[next_node] == 1 {
                        q.push_back(next_node);
                    }
                    next_edge = e.next;
                }
            }
        }

        q.into_iter().map(|v| v as i32).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
