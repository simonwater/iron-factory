use crate::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

/// [236. 二叉树的最近公共祖先](https://leetcode.cn/problems/lowest-common-ancestor-of-a-binary-tree/)
pub struct Solution;

impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return None;
        }
        let root_node = root.as_ref().unwrap();
        let p_node = p.as_ref().unwrap();
        let q_node = q.as_ref().unwrap();
        if Rc::ptr_eq(root_node, p_node) || Rc::ptr_eq(root_node, q_node) {
            return root;
        }

        let (left_child, right_child) = {
            let borrow = root_node.borrow();
            (
                borrow.left.as_ref().map(Rc::clone),
                borrow.right.as_ref().map(Rc::clone),
            )
        };

        let l = Self::lowest_common_ancestor(left_child, p.clone(), q.clone());
        let r = Self::lowest_common_ancestor(right_child, p.clone(), q.clone());

        if l.is_some() && r.is_some() {
            return root;
        }
        return if l.is_some() { l } else { r };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree;

    #[test]
    fn test1() {
        let root = tree![3, 5, 1, 6, 2, 0, 8, null, null, 7, 4];
        let node1 = TreeNode::find_node(&root, 5);
        let node2 = TreeNode::find_node(&root, 1);
        let ans = TreeNode::find_node(&root, 3);
        assert_eq!(Solution::lowest_common_ancestor(root, node1, node2), ans);

        let root = tree![3, 5, 1, 6, 2, 0, 8, null, null, 7, 4];
        let node1 = TreeNode::find_node(&root, 5);
        let node2 = TreeNode::find_node(&root, 4);
        let ans = TreeNode::find_node(&root, 5);
        assert_eq!(Solution::lowest_common_ancestor(root, node1, node2), ans);

        let root = tree![1, 2];
        let node1 = TreeNode::find_node(&root, 1);
        let node2 = TreeNode::find_node(&root, 2);
        let ans = TreeNode::find_node(&root, 1);
        assert_eq!(Solution::lowest_common_ancestor(root, node1, node2), ans);
    }
}
