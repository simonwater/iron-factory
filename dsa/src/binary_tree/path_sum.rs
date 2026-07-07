use crate::binary_tree::TreeNode;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Solution;

/// [112. 路径总和](https://leetcode.cn/problems/path-sum/description/)
impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        if root.is_none() {
            return false;
        }
        let borrow = root.as_ref().unwrap().borrow();
        if borrow.left.is_none() && borrow.right.is_none() && borrow.val == target_sum {
            return true;
        }
        Self::has_path_sum(borrow.left.clone(), target_sum - borrow.val)
            || Self::has_path_sum(borrow.right.clone(), target_sum - borrow.val)
    }

    /// [113. 路径总和 II](https://leetcode.cn/problems/path-sum-ii/description/)
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        let mut path = Vec::new();
        let mut ans = Vec::new();
        Self::dfs(root, target_sum, &mut path, &mut ans);
        ans
    }

    fn dfs(
        root: Option<Rc<RefCell<TreeNode>>>,
        target_sum: i32,
        path: &mut Vec<i32>,
        ans: &mut Vec<Vec<i32>>,
    ) {
        if let Some(node) = root {
            let borrow = node.borrow();
            path.push(borrow.val);
            if borrow.left.is_none() && borrow.right.is_none() && borrow.val == target_sum {
                ans.push(path.clone());
            } else {
                Self::dfs(borrow.left.clone(), target_sum - borrow.val, path, ans);
                Self::dfs(borrow.right.clone(), target_sum - borrow.val, path, ans);
            }
            path.pop();
        }
    }

    /// [437. 路径总和 III](https://leetcode.cn/problems/path-sum-iii/description/)
    pub fn path_sum_iii(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        let mut ans = 0;
        let mut map: HashMap<i64, i32> = HashMap::new();
        map.insert(0, 1);
        Self::path_sum_iii_dfs(root, target_sum as i64, 0, &mut ans, &mut map);
        ans
    }

    fn path_sum_iii_dfs(
        root: Option<Rc<RefCell<TreeNode>>>,
        target: i64,
        mut sum: i64,
        ans: &mut i32,
        map: &mut HashMap<i64, i32>,
    ) {
        if let Some(node) = root {
            let borrow = node.borrow();
            sum += borrow.val as i64;
            *ans += map.get(&(sum - target)).copied().unwrap_or(0);
            *map.entry(sum).or_insert(0) += 1;
            Self::path_sum_iii_dfs(borrow.left.clone(), target, sum, ans, map);
            Self::path_sum_iii_dfs(borrow.right.clone(), target, sum, ans, map);
            *map.get_mut(&sum).unwrap() -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree;

    #[test]
    fn test1() {
        let tree = tree![1, 2];
        assert_eq!(Solution::has_path_sum(tree, 1), false);

        let tree = tree![1, 2, 3];
        assert_eq!(Solution::has_path_sum(tree, 5), false);

        let tree = tree![];
        assert_eq!(Solution::has_path_sum(tree, 0), false);

        let tree = tree![5, 4, 8, 11, null, 13, 4, 7, 2, null, null, null, 1];
        assert_eq!(Solution::has_path_sum(tree, 22), true);
    }

    #[test]
    fn test2() {
        let root = tree![5, 4, 8, 11, null, 13, 4, 7, 2, null, null, 5, 1];
        let ans = vec![[5, 4, 11, 2], [5, 8, 4, 5]];
        assert_eq!(Solution::path_sum(root, 22), ans);

        let root = tree![1, 2, 3];
        let ans: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution::path_sum(root, 5), ans);
    }

    #[test]
    fn test3() {
        let root = tree![10, 5, -3, 3, 2, null, 11, 3, -2, null, 1];
        assert_eq!(Solution::path_sum_iii(root, 8), 3);

        let root = tree![5, 4, 8, 11, null, 13, 4, 7, 2, null, null, 5, 1];
        assert_eq!(Solution::path_sum_iii(root, 22), 3);
    }
}
