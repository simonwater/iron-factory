use crate::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

/// [104. 二叉树的最大深度](https://leetcode.cn/problems/maximum-depth-of-binary-tree/description/)
pub struct Solution;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            let borrow = node.borrow();
            let l_h = Self::max_depth(borrow.left.clone());
            let r_h = Self::max_depth(borrow.right.clone());
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
        let tree = tree![3, 9, 20, null, null, 15, 7];
        assert_eq!(Solution::max_depth(tree), 3);

        let tree = tree![1, null, 2];
        assert_eq!(Solution::max_depth(tree), 2);
    }
}
