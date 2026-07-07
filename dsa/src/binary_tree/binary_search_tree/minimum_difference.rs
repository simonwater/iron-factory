use crate::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

/// [530. 二叉搜索树的最小绝对差](https://leetcode.cn/problems/minimum-absolute-difference-in-bst/description/)
pub struct Solution;

impl Solution {
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut prev = i32::MIN / 2;
        let mut ans = i32::MAX;
        Self::dfs(&root, &mut prev, &mut ans);
        ans
    }

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, prev: &mut i32, ans: &mut i32) {
        if let Some(root_rc) = root {
            let node = root_rc.borrow();
            Self::dfs(&node.left, prev, ans);
            *ans = (*ans).min(node.val - (*prev));
            *prev = node.val;
            Self::dfs(&node.right, prev, ans);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
