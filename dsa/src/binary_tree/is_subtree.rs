use crate::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

/// [572. 另一棵树的子树](https://leetcode.cn/problems/subtree-of-another-tree/description/)
pub struct Solution;

impl Solution {
    pub fn is_subtree_simple(
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        Self::dfs_simple(&root, &sub_root)
    }

    fn dfs_simple(
        root: &Option<Rc<RefCell<TreeNode>>>,
        sub_root: &Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if root.is_none() {
            return false;
        }
        if Self::is_same_tree(root, sub_root) {
            return true;
        }
        Self::dfs_simple(&root.as_ref().unwrap().borrow().left, sub_root)
            || Self::dfs_simple(&root.as_ref().unwrap().borrow().right, sub_root)
    }

    pub fn is_subtree(
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        let sub_h = Self::get_depth(&sub_root);
        Self::dfs(&root, &sub_root, sub_h) == -1
    }

    fn dfs(
        root: &Option<Rc<RefCell<TreeNode>>>,
        sub_root: &Option<Rc<RefCell<TreeNode>>>,
        sub_h: i32,
    ) -> i32 {
        if let Some(root_node) = root {
            let root_borrow = root_node.borrow();
            let l_h = Self::dfs(&root_borrow.left, sub_root, sub_h);
            if l_h == -1 {
                return -1;
            }
            let r_h = Self::dfs(&root_borrow.right, sub_root, sub_h);
            if r_h == -1 {
                return -1;
            }
            let h = std::cmp::max(l_h, r_h) + 1;
            if h == sub_h && Self::is_same_tree(root, sub_root) {
                return -1;
            }
            h
        } else {
            0
        }
    }

    fn is_same_tree(p: &Option<Rc<RefCell<TreeNode>>>, q: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (p, q) {
            (None, None) => true,
            (Some(node1), Some(node2)) => {
                let borrow1 = node1.borrow();
                let borrow2 = node2.borrow();
                borrow1.val == borrow2.val
                    && Self::is_same_tree(&borrow1.left, &borrow2.left)
                    && Self::is_same_tree(&borrow1.right, &borrow2.right)
            }
            _ => false,
        }
    }

    fn get_depth(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            let borrow = node.borrow();
            let l_h = Self::get_depth(&borrow.left);
            let r_h = Self::get_depth(&borrow.right);
            std::cmp::max(l_h, r_h) + 1
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
        let tree1 = tree![3, 4, 5, 1, 2];
        let tree2 = tree![4, 1, 2];
        assert_eq!(Solution::is_subtree_simple(tree1, tree2), true);

        let tree1 = tree![3, 4, 5, 1, 2, null, null, null, null, 0];
        let tree2 = tree![4, 1, 2];
        assert_eq!(Solution::is_subtree_simple(tree1, tree2), false);

        let tree1 = tree![
            1, null, 1, null, 1, null, 1, null, 1, null, 1, null, 1, null, 1, null, 1, null, 1,
            null, 1, 2
        ];
        let tree2 = tree![1, null, 1, null, 1, null, 1, null, 1, null, 1, 2];
        assert_eq!(Solution::is_subtree_simple(tree1, tree2), true);
    }

    #[test]
    fn test2() {
        let tree1 = tree![3, 4, 5, 1, 2];
        let tree2 = tree![4, 1, 2];
        assert_eq!(Solution::is_subtree(tree1, tree2), true);

        let tree1 = tree![3, 4, 5, 1, 2, null, null, null, null, 0];
        let tree2 = tree![4, 1, 2];
        assert_eq!(Solution::is_subtree(tree1, tree2), false);

        let tree1 = tree![
            1, null, 1, null, 1, null, 1, null, 1, null, 1, null, 1, null, 1, null, 1, null, 1,
            null, 1, 2
        ];
        let tree2 = tree![1, null, 1, null, 1, null, 1, null, 1, null, 1, 2];
        assert_eq!(Solution::is_subtree(tree1, tree2), true);
    }
}
