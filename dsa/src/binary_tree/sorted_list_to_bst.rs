//! [109. 有序链表转换二叉搜索树](https://leetcode.cn/problems/convert-sorted-list-to-binary-search-tree/)
//!
use crate::binary_tree::TreeNode;
use crate::linkedlist::ListNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        if head.is_none() {
            return None;
        }
        let mut dummy = ListNode::new(0);
        dummy.next = head;
        let mut slow_steps = 0;
        let mut fast = dummy.next.as_deref();
        while let Some(node) = fast {
            if let Some(next_node) = node.next.as_deref() {
                slow_steps += 1;
                fast = next_node.next.as_deref();
            } else {
                break;
            }
        }
        let mut tail = &mut dummy;
        for _ in 0..slow_steps {
            tail = tail.next.as_deref_mut().unwrap();
        }

        let mut cur_node = tail.next.take().unwrap();
        let mut ans = TreeNode::new(cur_node.val);
        ans.left = Self::sorted_list_to_bst(dummy.next.take());
        ans.right = Self::sorted_list_to_bst(cur_node.next.take());

        Some(Rc::new(RefCell::new(ans)))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
