use crate::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

/// [226. 翻转二叉树](https://leetcode.cn/problems/invert-binary-tree/description/)
pub struct Solution;

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root.as_ref() {
            let mut node_borrow = node.borrow_mut();
            let l = node_borrow.left.take();
            let r = node_borrow.right.take();
            node_borrow.left = Self::invert_tree(r);
            node_borrow.right = Self::invert_tree(l);
        }
        root
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree;

    #[test]
    fn test1() {
        let tree1 = tree![4, 2, 7, 1, 3, 6, 9];
        let tree2 = tree![4, 7, 2, 9, 6, 3, 1];
        assert_eq!(Solution::invert_tree(tree1), tree2);

        let tree1 = tree![2, 1, 3];
        let tree2 = tree![2, 3, 1];
        assert_eq!(Solution::invert_tree(tree1), tree2);

        let tree1 = tree![];
        let tree2 = tree![];
        assert_eq!(Solution::invert_tree(tree1), tree2);
    }
}
