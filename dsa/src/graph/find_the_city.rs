//! [1334. 阈值距离内邻居最少的城市](https://leetcode.cn/problems/find-the-city-with-the-smallest-number-of-neighbors-at-a-threshold-distance/)
//!
//! 全源最短路径：佛洛伊德算法

use std::i32;

pub struct Solution;

impl Solution {
    pub fn find_the_city(n: i32, edges: Vec<Vec<i32>>, distance_threshold: i32) -> i32 {
        let n = n as usize;
        const INF: i32 = 0x3f3f3f3f;
        let mut g = vec![vec![INF; n]; n];
        for i in 0..n {
            g[i][i] = 0;
        }
        for e in edges {
            let (u, v, w) = (e[0] as usize, e[1] as usize, e[2]);
            g[u][v] = w;
            g[v][u] = w;
        }
        for k in 0..n {
            for i in 0..n {
                if g[k][i] == INF {
                    continue;
                }
                for j in 0..n {
                    let new_dist = g[i][k] + g[k][j];
                    if new_dist < g[i][j] {
                        g[i][j] = new_dist;
                    }
                }
            }
        }
        let mut ans = 0;
        let mut cnt = usize::MAX;
        for (i, nodes) in g.iter().enumerate() {
            let new_cnt = nodes.iter().filter(|&&v| v <= distance_threshold).count();
            if new_cnt <= cnt {
                cnt = new_cnt;
                ans = i;
            }
        }
        ans as i32
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
