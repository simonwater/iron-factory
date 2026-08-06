//! [538. 把二叉搜索树转换为累加树](https://leetcode.cn/problems/convert-bst-to-greater-tree/)
//!
use crate::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn convert_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut next = 0;
        Self::dfs(&root, &mut next);
        root
    }

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, next: &mut i32) {
        if let Some(root_rc) = root {
            let mut node = root_rc.borrow_mut();

            Self::dfs(&node.right, next);
            node.val += *next;
            *next = node.val;
            Self::dfs(&node.left, next);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
