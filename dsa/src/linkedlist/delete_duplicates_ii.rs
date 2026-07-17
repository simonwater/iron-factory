//! [82. 删除排序链表中的重复元素 II](https://leetcode.cn/problems/remove-duplicates-from-sorted-list-ii/)
//!

use crate::linkedlist::ListNode;
pub struct Solution;

impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        let mut p = &mut dummy;
        let mut cur = head;
        while let Some(mut node) = cur.take() {
            if let Some(mut next) = node.next.take() {
                if node.val == next.val {
                    let mut visitor = next.next.take();
                    while let Some(v_node) = visitor.as_mut() {
                        if v_node.val == node.val {
                            visitor = v_node.next.take();
                        } else {
                            break;
                        }
                    }
                    cur = visitor;
                } else {
                    p.next = Some(node);
                    cur = Some(next);
                    p = p.next.as_deref_mut().unwrap();
                }
            } else {
                p.next = Some(node);
            }
        }

        dummy.next
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
