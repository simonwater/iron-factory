/// [230. 二叉搜索树中第 K 小的元素](https://leetcode.cn/problems/kth-smallest-element-in-a-bst/)
///
use crate::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

/// 递归中序遍历
pub struct Solution;

impl Solution {
    //
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut k = k;
        Self::dfs(&root, &mut k)
    }

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, k: &mut i32) -> i32 {
        if let Some(root_rc) = root {
            let node = root_rc.borrow();
            let l = Self::dfs(&node.left, k);
            if l >= 0 {
                return l;
            }
            if *k == 1 {
                return node.val;
            }
            *k -= 1;
            return Self::dfs(&node.right, k);
        } else {
            -1
        }
    }
}

/// 迭代中序遍历
pub struct Solution2;

impl Solution2 {
    //
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        if root.is_none() {
            return -1;
        }
        let mut k = k;
        let mut visitor = root.clone();
        let mut stack = Vec::with_capacity(16);
        while !stack.is_empty() || visitor.is_some() {
            while let Some(node_rc) = visitor {
                stack.push(Rc::clone(&node_rc));
                let node = node_rc.borrow();
                visitor = node.left.clone();
            }
            let cur = stack.pop().unwrap();
            if k == 1 {
                return cur.borrow().val;
            }
            k -= 1;

            visitor = cur.borrow().right.clone();
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
