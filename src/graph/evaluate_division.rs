/// # [399. 除法求值](https://leetcode.cn/problems/evaluate-division/)
///
/// 使用二维数组记忆化中间结果时，需要区分“访问过不能再重复访问”但图上是联通的这种 “假不连通路径”。
/// 不能提前把状态记为-1。
struct Solution;

use std::collections::HashMap;
impl Solution {
    //
    pub fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        let mut node_map: HashMap<&String, usize> = HashMap::with_capacity(32);
        // 节点和索引的映射
        for e in equations.iter() {
            let (u, v) = (&e[0], &e[1]);
            let mut cnt = node_map.len();
            node_map.entry(u).or_insert(cnt);
            cnt = node_map.len();
            node_map.entry(v).or_insert(cnt);
        }
        // 创建邻接表
        let node_cnt = node_map.len();
        let mut states = vec![vec![0.0; node_cnt]; node_cnt];
        let mut g: Vec<Vec<(usize, f64)>> = vec![Vec::with_capacity(8); node_cnt];
        for (e, value) in equations.iter().zip(values) {
            let (&u, &v) = (node_map.get(&e[0]).unwrap(), node_map.get(&e[1]).unwrap());
            g[u].push((v, value));
            g[v].push((u, 1.0 / value));
            states[u][v] = value;
            states[v][u] = 1.0 / value;
        }

        let mut ans = vec![-1.0; queries.len()];
        let mut visited = vec![false; node_cnt];
        for (i, query) in queries.iter().enumerate() {
            if let (Some(&u), Some(&v)) = (node_map.get(&query[0]), node_map.get(&query[1])) {
                if u == v {
                    ans[i] = 1.0;
                } else if states[u][v] != 0.0 {
                    ans[i] = states[u][v];
                } else {
                    ans[i] = Self::dfs(u, v, &g, &mut states, &mut visited);
                }
            } else {
                ans[i] = -1.0;
            }
        }
        ans
    }

    fn dfs(
        start: usize,
        end: usize,
        g: &Vec<Vec<(usize, f64)>>,
        states: &mut Vec<Vec<f64>>,
        visited: &mut [bool],
    ) -> f64 {
        if start == end {
            return 1.0;
        }
        if states[start][end] != 0.0 {
            return states[start][end];
        }
        visited[start] = true;
        for &(next_node, value) in &g[start] {
            if !visited[next_node] {
                let next_value = Self::dfs(next_node, end, g, states, visited);
                if next_value != -1.0 {
                    states[start][end] = value * next_value;
                    break;
                }
            }
        }
        visited[start] = false;
        if states[start][end] == 0.0 {
            -1.0
        } else {
            states[start][end]
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test1() {
        let equations = vec![
            vec![String::from("x1"), String::from("x2")],
            vec![String::from("x2"), String::from("x3")],
            vec![String::from("x3"), String::from("x4")],
            vec![String::from("x4"), String::from("x5")],
        ];
        let values = vec![3.0, 4.0, 5.0, 6.0];
        let queries = vec![
            vec![String::from("x1"), String::from("x5")],
            vec![String::from("x5"), String::from("x2")],
            vec![String::from("x2"), String::from("x4")],
            vec![String::from("x2"), String::from("x2")],
            vec![String::from("x2"), String::from("x9")],
            vec![String::from("x9"), String::from("x9")],
        ];
        let ans = vec![360.0, 0.00833, 20.0, 1.0, -1.0, -1.0];
        assert_eq!(ans, Solution::calc_equation(equations, values, queries));
    }
}
