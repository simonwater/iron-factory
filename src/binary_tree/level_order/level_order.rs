use crate::binary_tree::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

/// [102. 二叉树的层序遍历](https://leetcode.cn/problems/binary-tree-level-order-traversal/description/)
struct Solution;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = Vec::new();
        if let Some(node) = root {
            let mut q = VecDeque::new();
            q.push_back(Rc::clone(&node));
            while !q.is_empty() {
                let sz = q.len();
                let mut level = Vec::with_capacity(sz);
                for _ in 0..sz {
                    if let Some(cur) = q.pop_front() {
                        let borrow = cur.borrow();
                        level.push(borrow.val);
                        if let Some(left) = borrow.left.as_ref() {
                            q.push_back(Rc::clone(left));
                        }
                        if let Some(right) = borrow.right.as_ref() {
                            q.push_back(Rc::clone(right));
                        }
                    }
                }
                ans.push(level);
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
        let root = tree![3, 9, 20, null, null, 15, 7];
        let ans = vec![vec![3], vec![9, 20], vec![15, 7]];
        assert_eq!(Solution::level_order(root), ans);

        let root = tree![1];
        let ans = vec![vec![1]];
        assert_eq!(Solution::level_order(root), ans);

        let root = tree![];
        let ans: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution::level_order(root), ans);
    }
}
