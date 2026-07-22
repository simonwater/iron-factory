//! [1584. 连接所有点的最小费用](https://leetcode.cn/problems/min-cost-to-connect-all-points/)
//!
//! prim算法基本策略：选定一个点作为已选节点集合中的起始点，不断把剩余点中距离已选点最近的点加入集合中，直到结束
//!
//! k算法基本策略：所有边从小到大排序，从最小的边开始往并查集里添加节点

/// K(Kruskal) 算法: 按边来处理，所有边按权重从小到大排序，然后遍历，往并查集中添加边的节点。
/// 适合边比较少的稀疏图
pub struct Solution;

impl Solution {
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        let mut edges = Vec::with_capacity(n * (n - 1) / 2);
        for (i, p1) in points.iter().enumerate() {
            for (j, p2) in points.iter().skip(i + 1).enumerate() {
                let w = (p1[0] - p2[0]).abs() + (p1[1] - p2[1]).abs();
                edges.push((i, i + j + 1, w));
            }
        }
        edges.sort_unstable_by_key(|a| a.2);
        let mut parents: Vec<usize> = (0..n).collect();
        let mut ans = 0;
        for (i, j, w) in edges {
            if Self::add(&mut parents, i, j) {
                // 存在新节点
                ans += w;
            }
        }
        ans
    }

    fn add(parents: &mut [usize], i: usize, j: usize) -> bool {
        let p_i = Self::find(parents, i);
        let p_j = Self::find(parents, j);
        if p_i != p_j {
            parents[p_j] = p_i;
            parents[j] = p_i;
            return true;
        }
        false
    }

    fn find(parents: &mut [usize], i: usize) -> usize {
        if parents[i] != i {
            parents[i] = Self::find(parents, parents[i]);
        }
        parents[i]
    }
}

/// P(Prim)算法：任选一个节点作为开始节点加入到最小生成树中，对于剩下的节点，每次都选出 **距离最小生成树最近**
/// 的点加入到树中，持续 `n - 1` 轮（边到数量）。
/// 
/// - 树中只有一个点时，很显然选择的是剩余点中距离这个点最近的点。
/// - 
pub struct Solution2;

impl Solution2 {
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        let mut visited = vec![false; n]; // 表示已经加入到结果中的点
        let mut dist = vec![i32::MAX; n]; // 树距离点的距离

        dist[0] = 0;
        let mut ans = 0;
        let mut cur_node = 0;
        let mut edge_cnt = 0;
        // 最终结果是 n - 1 条边，每次都从未选点中选距离最近的点构成一条边
        while edge_cnt < n - 1 {
            visited[cur_node] = true;
            let p1 = &points[cur_node];
            let mut next_node = 0;
            let mut min_edge = i32::MAX;
            for (i, p2) in points.iter().enumerate() {
                if !visited[i] {
                    let w = (p1[0] - p2[0]).abs() + (p1[1] - p2[1]).abs();
                    if w < dist[i] {
                        dist[i] = w;
                    }

                    if dist[i] < min_edge {
                        min_edge = dist[i];
                        next_node = i;
                    }
                }
            }
            cur_node = next_node;
            ans += min_edge;
            edge_cnt += 1;
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
