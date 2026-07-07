use crate::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

/// [513. 找树左下角的值](https://leetcode.cn/problems/find-bottom-left-tree-value/)
pub struct Solution;
use std::collections::VecDeque;
impl Solution {
    /// 按照先右后左的顺序入队列，最后弹出的节点就是树的左下角节点
    pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = -1;
        if let Some(root_node) = root.as_ref() {
            let mut q = VecDeque::new();
            q.push_back(Rc::clone(root_node));
            while !q.is_empty() {
                let node = q.pop_front().unwrap();
                let node_borrow = node.borrow();
                ans = node_borrow.val;
                let left = node_borrow.left.as_ref();
                let right = node_borrow.right.as_ref();
                if let Some(r_node) = right {
                    q.push_back(Rc::clone(r_node));
                }
                if let Some(l_node) = left {
                    q.push_back(Rc::clone(l_node));
                }
            }
        }
        ans
    }
}

pub struct Solution2;
impl Solution2 {
    /// 前序遍历访问树，记录当前访问到的最大深度，更新最大深度时记录刚访问到的节点，
    /// 该节点即为该深度的第一个节点，访问完后记下的节点即为结果。
    pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_depth = -1;
        let mut ans = -1;
        Self::dfs(&root, 0, &mut max_depth, &mut ans);
        ans
    }

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, depth: i32, max_depth: &mut i32, ans: &mut i32) {
        if let Some(node) = root {
            let borrow = node.borrow();
            if *max_depth < depth {
                *max_depth = depth;
                *ans = borrow.val;
            }
            Self::dfs(&borrow.left, depth + 1, max_depth, ans);
            Self::dfs(&borrow.right, depth + 1, max_depth, ans);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree;

    #[test]
    fn test1() {
        let tree = tree![2, 1, 3];
        let ans = 1;
        assert_eq!(Solution::find_bottom_left_value(tree), ans);

        let tree = tree![1, 2, 3, 4, null, 5, 6, null, null, 7];
        let ans = 7;
        assert_eq!(Solution::find_bottom_left_value(tree), ans);
    }

    #[test]
    fn test2() {
        let tree = tree![2, 1, 3];
        let ans = 1;
        assert_eq!(Solution2::find_bottom_left_value(tree), ans);

        let tree = tree![1, 2, 3, 4, null, 5, 6, null, null, 7];
        let ans = 7;
        assert_eq!(Solution2::find_bottom_left_value(tree), ans);
    }
}
