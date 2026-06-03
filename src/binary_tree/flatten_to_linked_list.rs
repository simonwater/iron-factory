use crate::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

/// [114. 二叉树展开为链表](https://leetcode.cn/problems/flatten-binary-tree-to-linked-list/)
struct Solution;
// 前序遍历
impl Solution {
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut dummy = Rc::new(RefCell::new(TreeNode::new(0)));
        let mut prev = dummy.clone();
        Self::dfs(root.take(), &mut prev);
        *root = dummy.borrow_mut().right.take();
    }

    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, prev: &mut Rc<RefCell<TreeNode>>) {
        if let Some(node_rc) = root {
            let (left, right) = {
                let mut node = node_rc.borrow_mut();
                (node.left.take(), node.right.take())
            };
            prev.borrow_mut().right = Some(node_rc.clone());
            *prev = node_rc;
            Self::dfs(left, prev);
            Self::dfs(right, prev);
        }
    }
}

struct Solution2;
// 逆前序遍历
impl Solution2 {
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut ans: Option<Rc<RefCell<TreeNode>>> = None;
        Self::dfs(root.take(), &mut ans);
        *root = ans;
    }

    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, next: &mut Option<Rc<RefCell<TreeNode>>>) {
        if let Some(node_rc) = root {
            let (left, right) = {
                let mut node = node_rc.borrow_mut();
                (node.left.take(), node.right.take())
            };
            Self::dfs(right, next);
            Self::dfs(left, next);

            node_rc.borrow_mut().right = next.take();
            *next = Some(node_rc);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree;

    #[test]
    fn test1() {}
}
