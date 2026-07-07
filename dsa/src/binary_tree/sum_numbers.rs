use crate::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

/// [129. 求根节点到叶节点数字之和](https://leetcode.cn/problems/sum-root-to-leaf-numbers/description/)
pub struct Solution;

impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let num = 0;
        let mut ans = 0;
        Self::dfs(root, num, &mut ans);
        ans
    }

    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, mut num: i32, ans: &mut i32) {
        if let Some(node) = root {
            let borrow = node.borrow();
            num = num * 10 + borrow.val;
            if borrow.left.is_none() && borrow.right.is_none() {
                *ans += num;
            } else {
                Self::dfs(borrow.left.clone(), num, ans);
                Self::dfs(borrow.right.clone(), num, ans);
            }
        }
    }

    pub fn sum_numbers_clean(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::dfs2(root, 0)
    }

    fn dfs2(root: Option<Rc<RefCell<TreeNode>>>, mut num: i32) -> i32 {
        if let Some(node) = root {
            let borrow = node.borrow();
            let (val, left, right) = (borrow.val, borrow.left.clone(), borrow.right.clone());
            num = num * 10 + val;
            if left.is_none() && right.is_none() {
                num
            } else {
                Self::dfs2(left, num) + Self::dfs2(right, num)
            }
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree;

    #[test]
    fn test1() {
        let root = tree![1, 2, 3];
        assert_eq!(Solution::sum_numbers(root), 25);

        let root = tree![4, 9, 0, 5, 1];
        assert_eq!(Solution::sum_numbers(root), 1026);
    }

    #[test]
    fn test2() {
        let root = tree![1, 2, 3];
        assert_eq!(Solution::sum_numbers_clean(root), 25);

        let root = tree![4, 9, 0, 5, 1];
        assert_eq!(Solution::sum_numbers_clean(root), 1026);
    }
}
