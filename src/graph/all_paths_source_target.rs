/// # [797. 所有可能的路径](https://leetcode.cn/problems/all-paths-from-source-to-target/)
///

struct Solution;

impl Solution {
    //
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ans = Vec::with_capacity(64);
        let mut path = Vec::with_capacity(16);
        path.push(0);
        Self::dfs(0, graph.len() - 1, &graph, &mut path, &mut ans);
        ans
    }

    fn dfs(
        start: usize,
        end: usize,
        g: &Vec<Vec<i32>>,
        path: &mut Vec<i32>,
        ans: &mut Vec<Vec<i32>>,
    ) {
        if start == end {
            ans.push(path.clone());
            return;
        }
        for &next in &g[start] {
            path.push(next);
            Self::dfs(next as usize, end, g, path, ans);
            path.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
