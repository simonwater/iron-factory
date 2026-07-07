use crate::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

/// [543. 二叉树的直径](https://leetcode.cn/problems/diameter-of-binary-tree/description/)
pub struct Solution;

impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = 0;
        Self::get_height(&root, &mut ans);
        ans
    }

    fn get_height(root: &Option<Rc<RefCell<TreeNode>>>, ans: &mut i32) -> i32 {
        if let Some(node) = root {
            let borrow = node.borrow();
            let l_h = Self::get_height(&borrow.left, ans);
            let r_h = Self::get_height(&borrow.right, ans);
            *ans = std::cmp::max(*ans, l_h + r_h);
            std::cmp::max(l_h, r_h) + 1
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree;

    #[test]
    fn test1() {
        let tree = tree![1, 2, 3, 4, 5];
        assert_eq!(Solution::diameter_of_binary_tree(tree), 3);

        let tree = tree![1, 2];
        assert_eq!(Solution::diameter_of_binary_tree(tree), 1);
    }
}
