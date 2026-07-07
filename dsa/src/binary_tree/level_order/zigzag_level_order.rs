use crate::binary_tree::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

/// [103. 二叉树的锯齿形层序遍历](https://leetcode.cn/problems/binary-tree-zigzag-level-order-traversal/)
pub struct Solution;

impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        if let Some(root_node) = root {
            let mut q = VecDeque::new();
            q.push_back(Rc::clone(&root_node));
            let mut is_rev = false;
            while !q.is_empty() {
                let sz = q.len();
                let mut level = Vec::with_capacity(sz);
                for _ in 0..sz {
                    let cur = q.pop_front().unwrap();
                    let borrow = cur.borrow();
                    level.push(borrow.val);
                    let left = borrow.left.as_ref();
                    let right = borrow.right.as_ref();
                    if let Some(left_node) = left {
                        q.push_back(Rc::clone(left_node));
                    }
                    if let Some(right_node) = right {
                        q.push_back(Rc::clone(right_node));
                    }
                }
                if is_rev {
                    level.reverse();
                }
                ans.push(level);
                is_rev = !is_rev;
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
    fn test1() {
        let tree = tree![3, 9, 20, null, null, 15, 7];
        let ans = vec![vec![3], vec![20, 9], vec![15, 7]];
        assert_eq!(Solution::zigzag_level_order(tree), ans);

        let tree = tree![1];
        let ans = vec![vec![1]];
        assert_eq!(Solution::zigzag_level_order(tree), ans);

        let tree = tree![];
        let ans: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution::zigzag_level_order(tree), ans);
    }
}
