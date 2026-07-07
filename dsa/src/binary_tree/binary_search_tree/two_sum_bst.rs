/// [653. 两数之和 IV - 输入二叉搜索树](https://leetcode.cn/problems/two-sum-iv-input-is-a-bst/)
use crate::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

/// 哈希表 + 任意顺序遍历
pub struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        let mut set = HashSet::with_capacity(32);
        Self::dfs(&root, k, &mut set)
    }

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, k: i32, set: &mut HashSet<i32>) -> bool {
        if let Some(root_rc) = root {
            let node = root_rc.borrow();
            if set.contains(&(k - node.val)) {
                return true;
            }
            set.insert(node.val);
            if Self::dfs(&node.left, k, set) {
                return true;
            }
            return Self::dfs(&node.right, k, set);
        } else {
            false
        }
    }
}

pub struct Solution2;

impl Solution2 {
    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        if root.is_none() {
            return false;
        }
        let mut stack1 = Vec::with_capacity(32);
        let mut stack2 = Vec::with_capacity(32);
        Self::en_stack1(root.as_ref().map(Rc::clone), &mut stack1);
        Self::en_stack2(root.as_ref().map(Rc::clone), &mut stack2);
        while !stack1.is_empty() && !stack2.is_empty() {
            let node_rc1 = stack1.last().unwrap();
            let node_rc2 = stack2.last().unwrap();
            if Rc::ptr_eq(node_rc1, node_rc2) {
                return false;
            }
            let node1 = node_rc1.borrow();
            let node2 = node_rc2.borrow();
            let sum = node1.val + node2.val;
            if sum == k {
                return true;
            } else if sum < k {
                let node = node1.right.as_ref().map(Rc::clone);
                drop(node1);
                stack1.pop();
                Self::en_stack1(node, &mut stack1);
            } else {
                let node = node2.left.as_ref().map(Rc::clone);
                drop(node2);
                stack2.pop();
                Self::en_stack2(node, &mut stack2);
            }
        }
        false
    }

    fn en_stack1(
        mut visitor: Option<Rc<RefCell<TreeNode>>>,
        stack: &mut Vec<Rc<RefCell<TreeNode>>>,
    ) {
        while let Some(node_rc) = visitor {
            stack.push(Rc::clone(&node_rc));
            visitor = node_rc.borrow().left.as_ref().map(Rc::clone);
        }
    }

    fn en_stack2(
        mut visitor: Option<Rc<RefCell<TreeNode>>>,
        stack: &mut Vec<Rc<RefCell<TreeNode>>>,
    ) {
        while let Some(node_rc) = visitor {
            stack.push(Rc::clone(&node_rc));
            visitor = node_rc.borrow().right.as_ref().map(Rc::clone);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
