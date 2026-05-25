use crate::binary_tree::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

/// [515. 在每个树行中找最大值](https://leetcode.cn/problems/find-largest-value-in-each-tree-row/)
struct Solution1;

impl Solution1 {
    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if root.is_none() {
            return vec![];
        }
        let mut q = VecDeque::new();
        let root_node = root.as_ref().unwrap();
        q.push_back(Rc::clone(root_node));
        let mut ans = Vec::new();
        while !q.is_empty() {
            let sz = q.len();
            let mut cur = i32::MIN;
            for _ in 0..sz {
                let node = q.pop_front().unwrap();
                let borrow = node.borrow();
                cur = cur.max(borrow.val);
                let left = borrow.left.as_ref();
                let right = borrow.right.as_ref();
                if let Some(left_node) = left {
                    q.push_back(Rc::clone(left_node));
                }
                if let Some(right_node) = right {
                    q.push_back(Rc::clone(right_node));
                }
            }
            ans.push(cur);
        }

        ans
    }
}

struct Solution2;

impl Solution2 {
    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = Vec::new();
        Self::dfs(&root, 0, &mut ans);
        ans
    }

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, depth: usize, ans: &mut Vec<i32>) {
        if let Some(node) = root {
            let borrow = node.borrow();
            if ans.len() == depth {
                ans.push(borrow.val);
            } else {
                ans[depth] = ans[depth].max(borrow.val);
            }
            let left = &borrow.left;
            let right = &borrow.right;
            Self::dfs(&left, depth + 1, ans);
            Self::dfs(&right, depth + 1, ans);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree;

    #[test]
    fn test1() {
        let tree = tree![1, 3, 2, 5, 3, null, 9];
        let ans = vec![1, 3, 9];
        assert_eq!(Solution1::largest_values(tree), ans);

        let tree = tree![1, 2, 3];
        let ans = vec![1, 3];
        assert_eq!(Solution1::largest_values(tree), ans);
    }

    #[test]
    fn test2() {
        let tree = tree![1, 3, 2, 5, 3, null, 9];
        let ans = vec![1, 3, 9];
        assert_eq!(Solution2::largest_values(tree), ans);

        let tree = tree![1, 2, 3];
        let ans = vec![1, 3];
        assert_eq!(Solution2::largest_values(tree), ans);
    }
}
