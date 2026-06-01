/// [547. 省份数量](https://leetcode.cn/problems/number-of-provinces/)
struct Solution;

impl Solution {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let mut n = is_connected.len();
        let mut ids = vec![0; n];
        for (idx, val) in ids.iter_mut().enumerate() {
            *val = idx;
        }
        for i in 0..is_connected.len() - 1 {
            for j in i + 1..is_connected.len() {
                if is_connected[i][j] == 1 {
                    let i_id = Self::find_id(i, &mut ids); // find内部可能改变父节点，保险期间每次都重新获取
                    let j_id = Self::find_id(j, &mut ids);
                    if i_id != j_id {
                        ids[j_id] = i_id;
                        n -= 1;
                    }
                }
            }
        }
        n as i32
    }

    fn find_id(mut node: usize, ids: &mut [usize]) -> usize {
        let old_node = node;
        while ids[node] != node {
            node = ids[node];
        }
        ids[old_node] = node; // 压缩路径
        node
    }
}

struct Solution2;

impl Solution2 {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let n = is_connected.len();
        let mut visited = vec![false; n];
        let mut ans = 0;
        for node in 0..n {
            if !visited[node] {
                ans += 1;
                Self::dfs(node, &is_connected, &mut visited);
            }
        }
        ans
    }

    fn dfs(node: usize, is_connected: &[Vec<i32>], visited: &mut [bool]) {
        visited[node] = true;
        for (next, &state) in is_connected[node].iter().enumerate() {
            if state == 1 && !visited[next] {
                Self::dfs(next, is_connected, visited);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
