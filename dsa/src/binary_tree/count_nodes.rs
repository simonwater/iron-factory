use crate::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

/// [222. 完全二叉树的节点个数](https://leetcode.cn/problems/count-complete-tree-nodes/description/)
pub struct Solution;

impl Solution {
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            let borrow = node.borrow();
            Self::count_nodes(borrow.left.clone()) + Self::count_nodes(borrow.right.clone()) + 1
        } else {
            0
        }
    }

    pub fn count_nodes_better(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            let borrow = node.borrow();
            let l_h = Self::get_height_iter(&borrow.left);
            let r_h = Self::get_height_iter(&borrow.right);
            if l_h == r_h {
                (1 << l_h) + Self::count_nodes_better(borrow.right.clone())
            } else {
                (1 << r_h) + Self::count_nodes_better(borrow.left.clone())
            }
        } else {
            0
        }
    }

    pub fn get_height(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            Self::get_height(&node.borrow().left) + 1
        } else {
            0
        }
    }

    fn get_height_iter(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = 0;
        let mut cur = root.clone();
        while let Some(node) = cur {
            ans += 1;
            cur = node.borrow().left.clone();
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree;

    #[test]
    fn test1() {
        let tree = tree![];
        assert_eq!(Solution::count_nodes(tree), 0);

        let tree = tree![1];
        assert_eq!(Solution::count_nodes(tree), 1);

        let tree = tree![1, 2, 3, 4, 5, 6];
        assert_eq!(Solution::count_nodes(tree), 6);
    }

    #[test]
    fn test2() {
        let tree = tree![];
        assert_eq!(Solution::count_nodes_better(tree), 0);

        let tree = tree![1];
        assert_eq!(Solution::count_nodes_better(tree), 1);

        let tree = tree![1, 2, 3, 4, 5, 6];
        assert_eq!(Solution::count_nodes_better(tree), 6);
    }
}
