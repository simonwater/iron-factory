/// [77. 组合](https://leetcode.cn/problems/combinations/)
pub struct Solution;
impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut ans = Vec::with_capacity(16);
        let mut path = Vec::with_capacity(k as usize);
        Self::dfs(1, &mut path, &mut ans, n, k);
        ans
    }

    fn dfs(i: i32, path: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>, n: i32, left: i32) {
        if left == 0 {
            ans.push(path.clone());
            return;
        }
        if n - i + 1 < left {
            // 剩余数已经不够填充满path
            return;
        }
        Self::dfs(i + 1, path, ans, n, left);
        path.push(i);
        Self::dfs(i + 1, path, ans, n, left - 1);
        path.pop();
    }
}

pub struct Solution2;
impl Solution2 {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut ans = Vec::with_capacity(16);
        let mut path = Vec::with_capacity(k as usize);
        Self::dfs(1, &mut path, &mut ans, n, k);
        ans
    }

    fn dfs(start: i32, path: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>, n: i32, left: i32) {
        if left == 0 {
            ans.push(path.clone());
            return;
        }
        for i in start..=n - left + 1 {
            path.push(i);
            Self::dfs(i + 1, path, ans, n, left - 1);
            path.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
