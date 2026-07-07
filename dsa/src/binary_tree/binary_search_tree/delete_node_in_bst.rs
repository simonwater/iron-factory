/// [450. 删除二叉搜索树中的节点](https://leetcode.cn/problems/delete-node-in-a-bst/)
///
use crate::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

/// 递归解
pub struct Solution;

impl Solution {
    //
    pub fn delete_node(
        root: Option<Rc<RefCell<TreeNode>>>,
        key: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node_rc) = root {
            let mut node = node_rc.borrow_mut();
            if key > node.val {
                node.right = Self::delete_node(node.right.take(), key);
                drop(node);
                return Some(node_rc);
            } else if key < node.val {
                node.left = Self::delete_node(node.left.take(), key);
                drop(node);
                return Some(node_rc);
            } else {
                let left = node.left.take();
                let right = node.right.take();
                drop(node);
                if left.is_none() || right.is_none() {
                    return left.or(right);
                }
                // 从右子树中拿出最小节点作为新的根
                let (new_right, min_node) = Self::delete_min_node(right);
                if let Some(node_rc) = min_node {
                    let mut node = node_rc.borrow_mut();
                    node.left = left;
                    node.right = new_right;
                    drop(node);
                    return Some(node_rc);
                }
            }
        }
        None
    }

    fn delete_min_node(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> (Option<Rc<RefCell<TreeNode>>>, Option<Rc<RefCell<TreeNode>>>) {
        if let Some(node_rc) = root {
            let mut node = node_rc.borrow_mut();
            if node.left.is_none() {
                let right = node.right.take();
                drop(node);
                return (right, Some(node_rc));
            } else {
                let (new_left, min) = Self::delete_min_node(node.left.take());
                node.left = new_left;
                drop(node);
                return (Some(node_rc), min);
            }
        } else {
            (None, None)
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
