/// [LCR 053. 二叉搜索树中的中序后继](https://leetcode.cn/problems/P5rCT8/)
///
use crate::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

/// 递归中序遍历
struct Solution;

impl Solution {
    //
    pub fn inorder_successor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut prev = None;
        Self::dfs(&root, &p, &mut prev)
    }

    fn dfs(
        root: &Option<Rc<RefCell<TreeNode>>>,
        p: &Option<Rc<RefCell<TreeNode>>>,
        prev: &mut Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(root_rc) = root {
            let node = root_rc.borrow();
            let l = Self::dfs(&node.left, p, prev);
            if l.is_some() {
                return l;
            }
            if prev == p {
                return root.clone();
            }
            *prev = root.clone();
            return Self::dfs(&node.right, p, prev);
        } else {
            None
        }
    }
}

/// 递归逆中序遍历
struct Solution2;
impl Solution2 {
    pub fn inorder_successor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut ans = None;
        Self::dfs(&root, &p, &mut ans);
        ans
    }

    fn dfs(
        root: &Option<Rc<RefCell<TreeNode>>>,
        p: &Option<Rc<RefCell<TreeNode>>>,
        next: &mut Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if let Some(root_rc) = root {
            let node = root_rc.borrow();
            if Self::dfs(&node.right, p, next) {
                return true;
            }
            if p == root {
                return true;
            }
            *next = root.clone();
            Self::dfs(&node.left, p, next)
        } else {
            false
        }
    }
}

/// 迭代中序遍历
struct Solution3;
impl Solution3 {
    pub fn inorder_successor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut prev = None;
        let mut stack = Vec::with_capacity(16);
        let mut visitor = root.clone();
        while !stack.is_empty() || visitor.is_some() {
            while let Some(node_rc) = visitor {
                visitor = {
                    let node = node_rc.borrow();
                    node.left.clone()
                };
                stack.push(node_rc);
            }
            let cur = stack.pop();
            if prev == p {
                return cur;
            }
            visitor = cur.as_ref().unwrap().borrow().right.clone();
            prev = cur;
        }

        None
    }
}

/// 二叉搜索树递归查找
struct Solution4;
impl Solution4 {
    pub fn inorder_successor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let target = p.as_ref().unwrap().borrow().val;
        let mut ans = None;
        Self::dfs(&root, target, &mut ans);
        ans
    }

    fn dfs(
        root: &Option<Rc<RefCell<TreeNode>>>,
        target: i32,
        next: &mut Option<Rc<RefCell<TreeNode>>>,
    ) {
        if let Some(root_rc) = root {
            let node = root_rc.borrow();
            if target >= node.val {
                Self::dfs(&node.right, target, next);
            } else {
                *next = root.clone();
                Self::dfs(&node.left, target, next);
            }
        }
    }
}

/// 二叉搜索树迭代查找
struct Solution5;
impl Solution5 {
    pub fn inorder_successor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let target = p.as_ref().unwrap().borrow().val;
        let mut ans = None;
        let mut visitor = root.clone();
        while let Some(node_rc) = visitor {
            let node = node_rc.borrow();
            if target >= node.val {
                // 往右查找
                visitor = node.right.clone();
            } else {
                // 往左查找
                visitor = node.left.clone();
                drop(node);
                ans = Some(node_rc); // 记录备选答案
            }
        }
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
