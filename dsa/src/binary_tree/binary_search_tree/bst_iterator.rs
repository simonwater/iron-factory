//! [173. 二叉搜索树迭代器](https://leetcode.cn/problems/binary-search-tree-iterator/)
//!
use crate::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

/// 用栈存储数据状态，栈顶永远放着下一个要遍历到的节点
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

/// 转化为链表，每次调用next都直接消耗头节点
pub struct BSTIterator2 {
    head: Option<Rc<RefCell<TreeNode>>>,
}

impl BSTIterator2 {
    pub fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let dummy = Rc::new(RefCell::new(TreeNode::new(-1)));
        let mut prev = dummy.clone();
        let mut visitor = root;
        while let Some(visitor_rc) = visitor {
            let left = visitor_rc.borrow_mut().left.take();
            if let Some(left_rc) = left {
                // 当前节点挂到left上
                let mut p = left_rc.clone();
                loop {
                    let next = p.borrow().right.clone();
                    if let Some(rc) = next {
                        p = rc;
                    } else {
                        break;
                    }
                }
                p.borrow_mut().right = Some(visitor_rc);
                visitor = Some(left_rc);
            } else {
                prev.borrow_mut().right = Some(visitor_rc.clone());
                visitor = visitor_rc.borrow().right.clone();
                prev = visitor_rc;
            }
        }
        Self {
            head: dummy.borrow_mut().right.take(),
        }
    }

    pub fn next(&mut self) -> i32 {
        if let Some(node_rc) = self.head.take() {
            self.head = node_rc.borrow_mut().right.take();
            return node_rc.borrow().val;
        }
        unreachable!()
    }

    pub fn has_next(&self) -> bool {
        self.head.is_some()
    }
}
