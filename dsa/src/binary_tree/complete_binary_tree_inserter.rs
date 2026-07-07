use crate::binary_tree::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

/// [919. 完全二叉树插入器](https://leetcode.cn/problems/complete-binary-tree-inserter/)
pub struct CBTInserter {
    root: Option<Rc<RefCell<TreeNode>>>,
    q: VecDeque<Rc<RefCell<TreeNode>>>,
}

impl CBTInserter {
    pub fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        if root.is_none() {
            unreachable!();
        }
        let mut q = VecDeque::with_capacity(16);
        let root_rc = root.as_ref().map(Rc::clone).unwrap();
        q.push_back(root_rc);
        // 以下处理保证了队列中的节点缺失左/右孩子中的一个或两个
        while let Some(node_rc) = q.front_mut() {
            let (left, right, is_complete) = {
                let node = node_rc.borrow();
                (
                    node.left.as_ref().map(Rc::clone),
                    node.right.as_ref().map(Rc::clone),
                    node.right.is_some(),
                )
            };
            if let Some(left_rc) = left {
                q.push_back(left_rc);
            }
            if let Some(right_rc) = right {
                q.push_back(right_rc);
                q.pop_front();
            }

            if !is_complete {
                break;
            }
        }

        Self { root, q }
    }

    pub fn insert(&mut self, val: i32) -> i32 {
        let node_rc = Rc::new(RefCell::new(TreeNode::new(val)));
        self.q.push_back(Rc::clone(&node_rc));
        let parent_rc = self.q.front().unwrap();
        let (p_val, has_left) = {
            let parent_node = parent_rc.borrow();
            (parent_node.val, parent_node.left.is_some())
        };
        if has_left {
            parent_rc.borrow_mut().right = Some(node_rc);
            self.q.pop_front();
        } else {
            parent_rc.borrow_mut().left = Some(node_rc);
        }

        p_val
    }

    pub fn get_root(&self) -> Option<Rc<RefCell<TreeNode>>> {
        self.root.as_ref().map(Rc::clone)
    }
}
