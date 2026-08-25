//! [2095. 删除链表的中间节点](https://leetcode.cn/problems/delete-the-middle-node-of-a-linked-list/)
//!
use crate::linkedlist::ListNode;
pub struct Solution;

impl Solution {
    pub fn delete_middle(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        dummy.next = head;
        let mut steps = 0;
        let mut fast = dummy.next.as_deref();
        while let Some(f) = fast {
            let Some(f_next) = f.next.as_deref() else {
                break;
            };
            fast = f_next.next.as_deref();
            steps += 1;
        }

        let mut slow = &mut dummy;
        for _ in 0..steps {
            slow = slow.next.as_deref_mut().unwrap();
        }
        let mid = slow.next.take();
        slow.next = mid.and_then(|mut l| l.next.take());
        dummy.next
    }
}
