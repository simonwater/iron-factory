/// [98. 验证二叉搜索树](https://leetcode.cn/problems/validate-binary-search-tree/)
///
use crate::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

// 自顶向下前序遍历
impl Solution {
    //
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::dfs(&root, i64::MIN, i64::MAX)
    }

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, min: i64, max: i64) -> bool {
        if let Some(node_rc) = root {
            let node = node_rc.borrow();
            let val = node.val as i64;
            if val <= min || val >= max {
                return false;
            }
            Self::dfs(&node.left, min, val) && Self::dfs(&node.right, val, max)
        } else {
            true
        }
    }
}

struct Solution2;

// 递归中序遍历
impl Solution2 {
    //
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut prev = i64::MIN;
        Self::dfs(&root, &mut prev)
    }

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, prev: &mut i64) -> bool {
        if let Some(node_rc) = root {
            let node = node_rc.borrow();
            if !Self::dfs(&node.left, prev) {
                return false;
            }
            let val = node.val as i64;
            if val <= *prev {
                return false;
            }
            *prev = val;
            Self::dfs(&node.right, prev)
        } else {
            true
        }
    }
}

struct Solution3;

// 迭代中序遍历
impl Solution3 {
    //
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
            return true;
        }
        let mut prev = i64::MIN;
        let mut stack = Vec::with_capacity(32);
        let mut visitor = root;
        while !stack.is_empty() || visitor.is_some() {
            while let Some(node_rc) = visitor {
                stack.push(Rc::clone(&node_rc));
                let node = node_rc.borrow();
                visitor = node.left.clone();
            }
            let cur_node_rc = stack.pop().unwrap();
            let cur_node = cur_node_rc.borrow();
            let val = cur_node.val as i64;
            if val <= prev {
                return false;
            }
            prev = val;
            visitor = cur_node.right.clone();
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree;

    #[test]
    fn test1() {}
}
