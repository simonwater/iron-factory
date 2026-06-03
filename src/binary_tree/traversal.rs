use crate::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

/// [144. 二叉树的前序遍历](https://leetcode.cn/problems/binary-tree-preorder-traversal/)
/// [94. 二叉树的中序遍历](https://leetcode.cn/problems/binary-tree-inorder-traversal/)
/// [145. 二叉树的后序遍历](https://leetcode.cn/problems/binary-tree-postorder-traversal/)
struct Solution;

/// 递归
impl Solution {
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = Vec::with_capacity(16);
        Self::pre_dfs(&root, &mut ans);
        ans
    }

    fn pre_dfs(root: &Option<Rc<RefCell<TreeNode>>>, ans: &mut Vec<i32>) {
        if let Some(root_rc) = root {
            let node = root_rc.borrow();
            ans.push(node.val);
            Self::pre_dfs(&node.left, ans);
            Self::pre_dfs(&node.right, ans);
        }
    }

    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = Vec::with_capacity(16);
        Self::in_dfs(&root, &mut ans);
        ans
    }

    fn in_dfs(root: &Option<Rc<RefCell<TreeNode>>>, ans: &mut Vec<i32>) {
        if let Some(root_rc) = root {
            let node = root_rc.borrow();
            Self::in_dfs(&node.left, ans);
            ans.push(node.val);
            Self::in_dfs(&node.right, ans);
        }
    }

    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = Vec::with_capacity(16);
        Self::post_dfs(&root, &mut ans);
        ans
    }

    fn post_dfs(root: &Option<Rc<RefCell<TreeNode>>>, ans: &mut Vec<i32>) {
        if let Some(root_rc) = root {
            let node = root_rc.borrow();
            Self::post_dfs(&node.left, ans);
            Self::post_dfs(&node.right, ans);
            ans.push(node.val);
        }
    }
}

struct Solution2;

/// 迭代
impl Solution2 {
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if root.is_none() {
            return vec![];
        }
        let mut ans = Vec::with_capacity(16);
        let mut stack = Vec::with_capacity(10);
        let root_rc = root.as_ref().unwrap();
        stack.push(Rc::clone(root_rc));
        while let Some(node_rc) = stack.pop() {
            let node = node_rc.borrow();
            ans.push(node.val);
            if let Some(right_rc) = node.right.as_ref() {
                stack.push(Rc::clone(right_rc));
            }
            if let Some(left_rc) = node.left.as_ref() {
                stack.push(Rc::clone(left_rc));
            }
        }
        ans
    }

    pub fn inorder_traversal(mut root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = Vec::with_capacity(16);
        let mut stack = Vec::with_capacity(10);
        while !stack.is_empty() || root.is_some() {
            while let Some(node_rc) = root {
                let node = node_rc.borrow();
                stack.push(Rc::clone(&node_rc));
                root = node.left.as_ref().map(Rc::clone);
            }
            let node_rc = stack.pop().unwrap();
            let node = node_rc.borrow();
            ans.push(node.val);
            root = node.right.as_ref().map(Rc::clone);
        }

        ans
    }

    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if root.is_none() {
            return vec![];
        }
        let mut ans = Vec::with_capacity(16);
        let mut stack = Vec::with_capacity(10);
        let root_rc = root.as_ref().unwrap();
        stack.push(Rc::clone(root_rc));
        while let Some(node_rc) = stack.pop() {
            let node = node_rc.borrow();
            ans.push(node.val);
            if let Some(left_rc) = node.left.as_ref() {
                stack.push(Rc::clone(left_rc));
            }
            if let Some(right_rc) = node.right.as_ref() {
                stack.push(Rc::clone(right_rc));
            }
        }

        ans.reverse();
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree;

    #[test]
    fn test1() {}
}
