/// [684. 冗余连接](https://leetcode.cn/problems/redundant-connection/)
/// 并查集，路径压缩
pub struct Solution;

impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges.len();
        let mut parents: Vec<usize> = (0..=n).collect();
        for edge in edges {
            let (u, v) = (edge[0] as usize, edge[1] as usize);
            let u_id = Self::find(u, &mut parents);
            let v_id = Self::find(v, &mut parents);
            if u_id == v_id {
                return edge;
            }
            parents[u_id] = v_id;
        }
        vec![]
    }

    fn find(val: usize, parents: &mut [usize]) -> usize {
        if parents[val] == val {
            return val;
        }
        parents[val] = Self::find(parents[val], parents);
        parents[val]
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
