/// # [990. 等式方程的可满足性](https://leetcode.cn/problems/satisfiability-of-equality-equations/)
///
/// **并查集**
///
/// 分两步:
/// 1. 所有通过 `==` 连接的节点归入对应的并查集分组里
/// 2. 所有通过 `!=` 连接的节点判断是否和并查集中的分组冲突
///
/// 必须通过两次循环来分开处理，一次循环内遇到 `!=` 时无法确定未来是否会遇到 `==`

pub struct Solution;

impl Solution {
    pub fn equations_possible(equations: Vec<String>) -> bool {
        let mut parents = [0usize; 26];
        for (i, val) in parents.iter_mut().enumerate() {
            *val = i;
        }

        for e in equations.iter().map(String::as_bytes) {
            if e[1] == b'=' {
                let u = (e[0] - b'a') as usize;
                let v = (e[3] - b'a') as usize;
                let u1 = Self::find(&mut parents, u);
                let v1 = Self::find(&mut parents, v);
                if u1 != v1 {
                    parents[u1] = v1;
                }
            }
        }

        for e in equations.iter().map(String::as_bytes) {
            if e[1] == b'!' {
                let u = (e[0] - b'a') as usize;
                let v = (e[3] - b'a') as usize;
                let u1 = Self::find(&mut parents, u);
                let v1 = Self::find(&mut parents, v);
                if u1 == v1 {
                    return false;
                }
            }
        }

        true
    }

    // 带路径压缩的查找
    fn find(parents: &mut [usize], u: usize) -> usize {
        if parents[u] != u {
            parents[u] = Self::find(parents, parents[u]);
        }
        parents[u]
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
