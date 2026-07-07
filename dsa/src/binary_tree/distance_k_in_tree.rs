use crate::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

/// [863. 二叉树中所有距离为 K 的结点](https://leetcode.cn/problems/all-nodes-distance-k-in-binary-tree/)
pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn distance_k(
        root: Option<Rc<RefCell<TreeNode>>>,
        target: Option<Rc<RefCell<TreeNode>>>,
        k: i32,
    ) -> Vec<i32> {
        if root.is_none() || target.is_none() {
            return Vec::new();
        }

        const INI: Option<Rc<RefCell<TreeNode>>> = None;
        let mut parents = [INI; 501];
        Self::init_parents(&root, &None, &mut parents);

        let mut visited = [false; 501];
        let mut q = VecDeque::new();
        let node = target.as_ref().unwrap();
        visited[node.borrow().val as usize] = true;
        q.push_back((Rc::clone(node), 0));

        let mut ans: Vec<i32> = Vec::with_capacity(16);
        while let Some((node_rc, dist)) = q.pop_front() {
            let node = node_rc.borrow();
            if dist == k {
                ans.push(node.val);
            } else {
                for rc_opt in [&node.left, &node.right, &parents[node.val as usize]] {
                    if let Some(rc) = rc_opt {
                        let val = rc.borrow().val;
                        if !visited[val as usize] {
                            visited[val as usize] = true;
                            q.push_back((Rc::clone(rc), dist + 1));
                        }
                    }
                }
            }
        }

        ans
    }

    fn init_parents(
        root: &Option<Rc<RefCell<TreeNode>>>,
        parent: &Option<Rc<RefCell<TreeNode>>>,
        parents: &mut [Option<Rc<RefCell<TreeNode>>>],
    ) {
        if let Some(root_rc) = root {
            let node = root_rc.borrow();
            if let Some(parent_rc) = parent {
                parents[node.val as usize] = Some(Rc::clone(parent_rc));
            }

            Self::init_parents(&node.left, root, parents);
            Self::init_parents(&node.right, root, parents);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
