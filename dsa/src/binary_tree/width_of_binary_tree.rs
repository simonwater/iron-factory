use crate::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

/// [662. 二叉树最大宽度](https://leetcode.cn/problems/maximum-width-of-binary-tree/)
pub struct Solution;

// dfs
impl Solution {
    pub fn width_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let mut ans = 1;
        Self::dfs(&root, 1, 0, &mut Vec::with_capacity(10), &mut ans);
        ans
    }

    fn dfs(
        root: &Option<Rc<RefCell<TreeNode>>>,
        cur_idx: i32,
        depth: usize,
        left_idx: &mut Vec<i32>,
        ans: &mut i32,
    ) {
        if let Some(node) = root {
            if left_idx.len() == depth {
                // 当前层第一次访问到
                left_idx.push(cur_idx);
            } else {
                *ans = *ans.max(&mut (cur_idx - left_idx[depth] + 1));
            }

            // 防止编号指数级膨胀，下一层整体向左平移。移动的步数等于当前层最左边节点的编号。
            let abs_idx = cur_idx - left_idx[depth];
            let borrow = node.borrow();
            Self::dfs(&borrow.left, abs_idx * 2, depth + 1, left_idx, ans);
            Self::dfs(&borrow.right, abs_idx * 2 + 1, depth + 1, left_idx, ans);
        }
    }
}

use std::collections::VecDeque;
// bfs层序遍历
pub struct Solution1;
impl Solution1 {
    pub fn width_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let mut q = VecDeque::new();
        let root_node = root.as_ref().unwrap();
        q.push_back((Rc::clone(root_node), 1));
        let mut ans = 1;
        while !q.is_empty() {
            let sz = q.len();
            let mut left_idx = 0;
            let mut right_idx = 0;
            // 防止二叉树退化为链表时高度太高导致溢出
            let bias = q.front().unwrap().1;
            for i in 0..sz {
                let (node, idx) = q.pop_front().unwrap();
                let abs_idx = idx - bias;
                if i == 0 {
                    left_idx = abs_idx;
                }
                if i == sz - 1 {
                    right_idx = abs_idx;
                }
                let (left, right) = {
                    let borrow = node.borrow();
                    (
                        borrow.left.as_ref().map(Rc::clone),
                        borrow.right.as_ref().map(Rc::clone),
                    )
                };
                if let Some(left_node) = left {
                    q.push_back((left_node, abs_idx * 2));
                }
                if let Some(right_node) = right {
                    q.push_back((right_node, abs_idx * 2 + 1));
                }
            }
            ans = ans.max(right_idx - left_idx + 1);
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
