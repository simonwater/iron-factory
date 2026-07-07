use crate::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

/// [814. 二叉树剪枝](https://leetcode.cn/problems/binary-tree-pruning/description/)
pub struct Solution;

impl Solution {
    pub fn prune_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        root.and_then(|node| {
            let mut borrow = node.borrow_mut();
            borrow.left = Self::prune_tree(borrow.left.take());
            borrow.right = Self::prune_tree(borrow.right.take());
            if borrow.left.is_none() && borrow.right.is_none() && borrow.val == 0 {
                None
            } else {
                drop(borrow);
                Some(node)
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree;

    #[test]
    fn test1() {
        let tree1 = tree![1, null, 0, 0, 1];
        let tree2 = tree![1, null, 0, null, 1];
        assert_eq!(Solution::prune_tree(tree1), tree2);

        let tree1 = tree![1, 0, 1, 0, 0, 0, 1];
        let tree2 = tree![1, null, 1, null, 1];
        assert_eq!(Solution::prune_tree(tree1), tree2);

        let tree1 = tree![1, 1, 0, 1, 1, 0, 1, 0];
        let tree2 = tree![1, 1, 0, 1, 1, null, 1];
        assert_eq!(Solution::prune_tree(tree1), tree2);
    }
}
