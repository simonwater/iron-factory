/// # [2097. 合法重新排列数对](https://leetcode.cn/problems/valid-arrangement-of-pairs/)
///
/// **有向图 欧拉通路 Hierholzer算法**

pub struct Solution;
use std::collections::HashMap;
impl Solution {
    pub fn valid_arrangement(mut pairs: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // 建图，计算度
        let mut g = HashMap::with_capacity(pairs.len());
        let mut degs = HashMap::with_capacity(pairs.len()); // key: [入度, 出度]
        for pair in &pairs {
            let (u, v) = (pair[0], pair[1]);
            g.entry(u).or_insert_with(|| Vec::with_capacity(8)).push(v);
            degs.entry(u).or_insert_with(|| vec![0; 2])[1] += 1; // 出度 +1
            degs.entry(v).or_insert_with(|| vec![0; 2])[0] += 1; // 入度 +1
        }

        // 寻找起点，环路的话起点任选。非环路起点是入度比出度小1的节点
        let mut start = pairs[0][0];
        for (&node, in_out) in degs.iter() {
            if in_out[1] - in_out[0] == 1 {
                start = node;
                break;
            }
        }

        // 搜索节点路径
        let mut path = Vec::with_capacity(g.len());
        Self::dfs(start, &mut g, &mut path);

        // 重新组织数对
        path.reverse();
        for i in 0..path.len() - 1 {
            pairs[i][0] = path[i];
            pairs[i][1] = path[i + 1];
        }
        pairs
    }

    fn dfs(cur: i32, g: &mut HashMap<i32, Vec<i32>>, ans: &mut Vec<i32>) {
        while let Some(nexts) = g.get_mut(&cur) {
            if let Some(next) = nexts.pop() {
                Self::dfs(next, g, ans);
            } else {
                break;
            }
        }
        ans.push(cur);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
