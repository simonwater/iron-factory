use crate::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

/// [110. 平衡二叉树](https://leetcode.cn/problems/balanced-binary-tree/description/)
pub struct Solution;

impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::dfs(&root) != -1
    }

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            let borrow = node.borrow();
            let l = Self::dfs(&borrow.left);
            if l == -1 {
                return -1;
            }
            let r = Self::dfs(&borrow.right);
            if r == -1 {
                return -1;
            }
            if (l - r).abs() > 1 {
                return -1;
            }
            std::cmp::max(l, r) + 1
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
        let tree = tree![3, 9, 20, null, null, 15, 7];
        assert_eq!(Solution::is_balanced(tree), true);

        let tree = tree![1, 2, 2, 3, 3, null, null, 4, 4];
        assert_eq!(Solution::is_balanced(tree), false);

        let tree = tree![];
        assert_eq!(Solution::is_balanced(tree), true);
    }
}
