use crate::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

/// [700. 二叉搜索树中的搜索](https://leetcode.cn/problems/search-in-a-binary-search-tree/)
struct Solution;
// 递归
impl Solution {
    pub fn search_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Self::dfs(&root, val)
    }

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        root.as_ref().and_then(|node_rc| {
            let node = node_rc.borrow();
            if node.val == val {
                return Some(Rc::clone(node_rc));
            } else if node.val < val {
                return Self::dfs(&node.right, val);
            }
            return Self::dfs(&node.left, val);
        })
    }
}

struct Solution2;
// 迭代
impl Solution2 {
    pub fn search_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut visitor = root.as_ref().map(Rc::clone);
        while let Some(node_rc) = visitor {
            let node = node_rc.borrow();
            if node.val == val {
                return Some(Rc::clone(&node_rc));
            } else if node.val < val {
                visitor = node.right.as_ref().map(Rc::clone);
            } else {
                visitor = node.left.as_ref().map(Rc::clone);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree;

    #[test]
    fn test1() {}
}
