/// # [51. N 皇后](https://leetcode.cn/problems/n-queens/)
///

pub struct Solution;

impl Solution {
    //
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let n = n as usize;
        let mut path = vec![0usize; n];
        let mut ans = Vec::with_capacity(32);
        Self::dfs(0, &mut path, &mut ans);
        ans
    }

    fn dfs(i: usize, path: &mut [usize], ans: &mut Vec<Vec<String>>) {
        if i == path.len() {
            ans.push(Self::make_solution(path));
            return;
        }
        for c in 0..path.len() {
            if Self::is_ok(i, c, path) {
                path[i] = c;
                Self::dfs(i + 1, path, ans);
            }
        }
    }

    fn is_ok(r: usize, c: usize, path: &mut [usize]) -> bool {
        for i in 0..r {
            if Self::is_attack(r as i32, c as i32, i as i32, path[i] as i32) {
                return false;
            }
        }
        true
    }

    fn is_attack(r1: i32, c1: i32, r2: i32, c2: i32) -> bool {
        return r1 == r2 || c1 == c2 || (r1 - r2).abs() == (c1 - c2).abs();
    }

    fn make_solution(path: &[usize]) -> Vec<String> {
        let n = path.len();
        let mut ans = Vec::with_capacity(n);
        for &c in path {
            let mut row = vec![b'.'; n];
            row[c] = b'Q';
            ans.push(String::from_utf8(row).unwrap());
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        Solution::solve_n_queens(1);
    }
}
