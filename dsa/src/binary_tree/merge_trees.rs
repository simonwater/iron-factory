use crate::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

/// [617. 合并二叉树](https://leetcode.cn/problems/merge-two-binary-trees/description/)
pub struct Solution;

impl Solution {
    pub fn merge_trees(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let (Some(n1), Some(n2)) = (&root1, &root2) {
            let val = n1.borrow().val + n2.borrow().val;
            let mut node = TreeNode::new(val);
            let l1 = n1.borrow_mut().left.take();
            let l2 = n2.borrow_mut().left.take();
            node.left = Self::merge_trees(l1, l2);

            let r1 = n1.borrow_mut().right.take();
            let r2 = n2.borrow_mut().right.take();
            node.right = Self::merge_trees(r1, r2);
            return Some(Rc::new(RefCell::new(node)));
        }
        root1.or(root2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree;

    #[test]
    fn test1() {
        let tree1 = tree![1, 3, 2, 5];
        let tree2 = tree![2, 1, 3, null, 4, null, 7];
        let tree3 = tree![3, 4, 5, 5, 4, null, 7];
        assert_eq!(Solution::merge_trees(tree1, tree2), tree3);

        let tree1 = tree![1];
        let tree2 = tree![1, 2];
        let tree3 = tree![2, 2];
        assert_eq!(Solution::merge_trees(tree1, tree2), tree3);
    }
}
