use crate::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

/// [124. 二叉树中的最大路径和](https://leetcode.cn/problems/binary-tree-maximum-path-sum/)
pub struct Solution;

impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = i32::MIN;
        Self::dfs(&root, &mut ans);
        ans
    }

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, ans: &mut i32) -> i32 {
        if let Some(root_rc) = root {
            let node = root_rc.borrow();
            let l = Self::dfs(&node.left, ans);
            let r = Self::dfs(&node.right, ans);
            *ans = (*ans).max(l + r + node.val);
            (l.max(r) + node.val).max(0)
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
