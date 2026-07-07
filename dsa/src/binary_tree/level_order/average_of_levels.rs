use crate::binary_tree::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

/// [637. 二叉树的层平均值](https://leetcode.cn/problems/average-of-levels-in-binary-tree/)
pub struct Solution;

impl Solution {
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        let mut ans = Vec::new();
        if let Some(root_node) = root {
            let mut q = VecDeque::new();
            q.push_back(Rc::clone(&root_node));
            while !q.is_empty() {
                let sz = q.len();
                let mut sum = 0f64;
                for _ in 0..sz {
                    if let Some(node) = q.pop_front() {
                        let borrow = node.borrow();
                        sum += borrow.val as f64;
                        let left = borrow.left.as_ref();
                        let right = borrow.right.as_ref();
                        if let Some(l_node) = left {
                            q.push_back(Rc::clone(l_node));
                        }
                        if let Some(r_node) = right {
                            q.push_back(Rc::clone(r_node));
                        }
                    }
                }
                ans.push(sum / sz as f64);
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
        let ans = vec![3.00000, 14.50000, 11.00000];
        assert_eq!(Solution::average_of_levels(tree), ans);

        let tree = tree![3, 9, 20, 15, 7];
        let ans = vec![3.00000, 14.50000, 11.00000];
        assert_eq!(Solution::average_of_levels(tree), ans);

        let tree = tree![2147483647, 2147483647, 2147483647];
        let ans = vec![2147483647.00000, 2147483647.00000];
        assert_eq!(Solution::average_of_levels(tree), ans);
    }
}
