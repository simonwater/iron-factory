use crate::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

/// [199. 二叉树的右视图](https://leetcode.cn/problems/binary-tree-right-side-view/)
pub struct Solution;

impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans: Vec<i32> = Vec::with_capacity(16);
        Self::dfs(&root, 0, &mut ans);
        ans
    }

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, depth: usize, ans: &mut Vec<i32>) {
        if let Some(root_rc) = root {
            let node = root_rc.borrow();
            if ans.len() == depth {
                ans.push(node.val);
            }
            Self::dfs(&node.right, depth + 1, ans);
            Self::dfs(&node.left, depth + 1, ans);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
