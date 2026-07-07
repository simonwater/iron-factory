use crate::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

/// [101. 对称二叉树](https://leetcode.cn/problems/symmetric-tree/description/)
pub struct Solution;

impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
            return true;
        }
        let borrow = root.as_ref().unwrap().borrow();
        Self::check(&borrow.left, &borrow.right)
    }

    fn check(root1: &Option<Rc<RefCell<TreeNode>>>, root2: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (root1, root2) {
            (None, None) => true,
            (None, _) => false,
            (_, None) => false,
            (Some(node1), Some(node2)) => {
                let borrow1 = node1.borrow();
                let borrow2 = node2.borrow();
                borrow1.val == borrow2.val
                    && Self::check(&borrow1.left, &borrow2.right)
                    && Self::check(&borrow1.right, &borrow2.left)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree;

    #[test]
    fn test1() {
        let tree = tree![1, 2, 2, 3, 4, 4, 3];
        assert_eq!(Solution::is_symmetric(tree), true);

        let tree = tree![1, 2, 2, null, 3, null, 3];
        assert_eq!(Solution::is_symmetric(tree), false);
    }
}
