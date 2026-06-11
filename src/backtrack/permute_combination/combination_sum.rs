/// [39. 组合总和](https://leetcode.cn/problems/combination-sum/description/)
/// 输入数字无重复、每个数可重复选

/// 从给定数组的视角，从头到尾做：选/不选 两个决策
struct Solution1;
impl Solution1 {
    pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut path = Vec::new();
        let mut ans = Vec::new();
        candidates.sort_unstable();
        Self::dfs(&candidates, 0, target, &mut path, &mut ans);
        ans
    }

    fn dfs(
        candidates: &[i32],
        i: usize,
        target: i32,
        path: &mut Vec<i32>,
        ans: &mut Vec<Vec<i32>>,
    ) {
        if target == 0 {
            ans.push(path.clone());
            return;
        }
        if i == candidates.len() || candidates[i] > target {
            return;
        }

        path.push(candidates[i]);
        Self::dfs(candidates, i, target - candidates[i], path, ans);
        path.pop();
        Self::dfs(candidates, i + 1, target, path, ans);
    }
}

/// 从最终答案的视角，每一个位置都枚举所有可能的数据
struct Solution2;
impl Solution2 {
    pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut path = Vec::new();
        let mut ans = Vec::new();
        candidates.sort_unstable();
        Self::dfs(&candidates, 0, target, &mut path, &mut ans);
        ans
    }

    fn dfs(
        candidates: &Vec<i32>,
        start: usize,
        target: i32,
        path: &mut Vec<i32>,
        ans: &mut Vec<Vec<i32>>,
    ) {
        if target == 0 {
            ans.push(path.clone());
            return;
        }

        for i in start..candidates.len() {
            let num = candidates[i];
            if num > target {
                break;
            }
            path.push(num);
            Self::dfs(candidates, i, target - num, path, ans);
            path.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let candidates = vec![2, 3, 6, 7];
        let target = 7;
        let ans = vec![vec![2, 2, 3], vec![7]];
        assert_eq!(Solution1::combination_sum(candidates, target), ans);

        let candidates = vec![2, 3, 5];
        let target = 8;
        let ans = vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]];
        assert_eq!(Solution1::combination_sum(candidates, target), ans);

        let candidates = vec![2];
        let target = 1;
        let ans: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution1::combination_sum(candidates, target), ans);
    }

    #[test]
    fn test2() {
        let candidates = vec![2, 3, 6, 7];
        let target = 7;
        let ans = vec![vec![2, 2, 3], vec![7]];
        assert_eq!(Solution2::combination_sum(candidates, target), ans);

        let candidates = vec![2, 3, 5];
        let target = 8;
        let ans = vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]];
        assert_eq!(Solution2::combination_sum(candidates, target), ans);

        let candidates = vec![2];
        let target = 1;
        let ans: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution2::combination_sum(candidates, target), ans);
    }
}
