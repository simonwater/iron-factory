//! [173. 二叉搜索树迭代器](https://leetcode.cn/problems/binary-search-tree-iterator/)
//!
use crate::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
pub struct BSTIterator {
    stack: Vec<Rc<RefCell<TreeNode>>>,
}

impl BSTIterator {
    pub fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut stack = Vec::with_capacity(16);
        Self::push_root(&mut stack, root);
        Self { stack }
    }

    fn push_root(stack: &mut Vec<Rc<RefCell<TreeNode>>>, mut root: Option<Rc<RefCell<TreeNode>>>) {
        while let Some(node) = root {
            root = node.borrow_mut().left.take();
            stack.push(node);
        }
    }

    pub fn next(&mut self) -> i32 {
        if let Some(node) = self.stack.pop() {
            let right = node.borrow_mut().right.take();
            Self::push_root(&mut self.stack, right);
            return node.borrow().val;
        }
        unreachable!()
    }

    pub fn has_next(&self) -> bool {
        !self.stack.is_empty()
    }
}
