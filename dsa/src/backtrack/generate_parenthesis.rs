/// # [22. 括号生成](https://leetcode.cn/problems/generate-parentheses/)
///

pub struct Solution;

impl Solution {
    //
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let n = n as usize;
        let mut ans = Vec::with_capacity(32);
        let mut path = vec![0u8; n * 2];
        Self::dfs(0, 0, 0, &mut path, &mut ans);
        ans
    }

    fn dfs(i: usize, left: usize, right: usize, path: &mut [u8], ans: &mut Vec<String>) {
        if i == path.len() {
            let s = String::from_utf8(path.to_vec()).unwrap();
            ans.push(s);
            return;
        }
        if left * 2 < path.len() {
            path[i] = b'(';
            Self::dfs(i + 1, left + 1, right, path, ans);
        }
        if right < left {
            path[i] = b')';
            Self::dfs(i + 1, left, right + 1, path, ans);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
