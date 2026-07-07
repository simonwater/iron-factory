/// # [1976. 到达目的地的方案数](https://leetcode.cn/problems/number-of-ways-to-arrive-at-destination/)
/// dijkstra算法
use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Solution;

impl Solution {
    pub fn count_paths(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let x = 1000000007;
        let mut g = vec![Vec::with_capacity(8); n];
        for road in roads {
            let (u, v, w) = (road[0] as usize, road[1] as usize, road[2] as i64);
            g[u].push((v, w));
            g[v].push((u, w));
        }
        let mut dist = vec![i64::MAX / 2; n];
        let mut ways = vec![0; n];
        dist[0] = 0;
        ways[0] = 1;

        let mut pq = BinaryHeap::with_capacity(200);
        pq.push(Reverse((0i64, 0usize))); // 0: dist; 1: node
        while let Some(Reverse((d, node))) = pq.pop() {
            if d > dist[node] {
                continue;
            }
            for &(next, w) in &g[node] {
                let new_dist = d + w;
                if new_dist < dist[next] {
                    dist[next] = new_dist;
                    ways[next] = ways[node];
                    pq.push(Reverse((new_dist, next)));
                } else if new_dist == dist[next] {
                    // 需要特殊注意相等时无需重复入队列的情况，直观看虽然距离相等，但更新了ways，所以是不是应该重入队列继续级联更新？
                    // 答案是不用，因为小顶堆的性质先出来的肯定是更小值，根据当前d + w == dist[next]，必然有：d < dist[next]，
                    // 所以next虽然有一个等于d + w的值，但必然大于当前出队列的d，next就肯定还在队列里，不用重复进队列
                    ways[next] = (ways[next] + ways[node]) % x;
                }
            }
        }
        ways[n - 1]
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
