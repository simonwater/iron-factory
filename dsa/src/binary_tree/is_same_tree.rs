use crate::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

/// [100. 相同的树](https://leetcode.cn/problems/same-tree/description/)
pub struct Solution;

impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (p, q) {
            (None, None) => true,
            (None, _) => false,
            (_, None) => false,
            (Some(node1), Some(node2)) => {
                let borrow1 = node1.borrow();
                let borrow2 = node2.borrow();
                borrow1.val == borrow2.val
                    && Self::is_same_tree(borrow1.left.clone(), borrow2.left.clone())
                    && Self::is_same_tree(borrow1.right.clone(), borrow2.right.clone())
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
        let tree1 = tree![1, 2, 3];
        let tree2 = tree![1, 2, 3];
        assert_eq!(Solution::is_same_tree(tree1, tree2), true);

        let tree1 = tree![1, 2];
        let tree2 = tree![1, null, 2];
        assert_eq!(Solution::is_same_tree(tree1, tree2), false);

        let tree1 = tree![1, 2, 1];
        let tree2 = tree![1, 1, 2];
        assert_eq!(Solution::is_same_tree(tree1, tree2), false);
    }
}
