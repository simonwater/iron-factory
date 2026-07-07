use crate::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

/// [235. 二叉搜索树的最近公共祖先](https://leetcode.cn/problems/lowest-common-ancestor-of-a-binary-search-tree/)
pub struct Solution;

impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut p_val = p.as_ref().unwrap().borrow().val;
        let mut q_val = q.as_ref().unwrap().borrow().val;
        if p_val > q_val {
            std::mem::swap(&mut p_val, &mut q_val);
        }

        Self::dfs(&root, p_val, q_val)
    }

    fn dfs(
        root: &Option<Rc<RefCell<TreeNode>>>,
        p_val: i32,
        q_val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(root_rc) = root {
            let node = root_rc.borrow();
            if node.val < p_val {
                return Self::dfs(&node.right, p_val, q_val);
            } else if node.val > q_val {
                return Self::dfs(&node.left, p_val, q_val);
            } else {
                return Some(Rc::clone(root_rc));
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
