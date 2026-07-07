/// [216. 组合总和 III](https://leetcode.cn/problems/combination-sum-iii/)
pub struct Solution;

impl Solution {
    //
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let k = k as usize;
        let mut ans = Vec::with_capacity(16);
        let mut path = Vec::with_capacity(k);
        Self::dfs(1, n, k, &mut path, &mut ans);
        ans
    }

    fn dfs(num: i32, target: i32, k: usize, path: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
        if target == 0 && path.len() == k {
            ans.push(path.clone());
            return;
        }
        // 剪枝
        let left_num_cnt = (10 - num) as usize;
        if num == 10 || num > target || path.len() >= k || path.len() + left_num_cnt < k {
            return;
        }
        path.push(num);
        Self::dfs(num + 1, target - num, k, path, ans);
        path.pop();

        Self::dfs(num + 1, target, k, path, ans);
    }
}

pub struct Solution2;

impl Solution2 {
    //
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let k = k as usize;
        let mut ans = Vec::with_capacity(16);
        let mut path = vec![0; k];
        Self::dfs(1, n, 0, &mut path, &mut ans);
        ans
    }

    fn dfs(start: i32, target: i32, pos: usize, path: &mut [i32], ans: &mut Vec<Vec<i32>>) {
        if target == 0 && pos == path.len() {
            ans.push(path.to_vec());
            return;
        }
        // 剪枝
        let left_num_cnt = 10 - (start as usize);
        if start == 10 || start > target || pos >= path.len() || pos + left_num_cnt < path.len() {
            return;
        }
        for num in start..10 {
            path[pos] = num;
            Self::dfs(num + 1, target - num, pos + 1, path, ans);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
