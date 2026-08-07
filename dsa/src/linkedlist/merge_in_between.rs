//! [1669. 合并两个链表](https://leetcode.cn/problems/merge-in-between-linked-lists/)
//!
use crate::linkedlist::ListNode;
pub struct Solution;

impl Solution {
    pub fn merge_in_between(
        mut list1: Option<Box<ListNode>>,
        a: i32,
        b: i32,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut p1 = list1.as_deref_mut().unwrap();
        for _ in 1..a {
            p1 = p1.next.as_deref_mut().unwrap();
        }

        let mut del_head = p1.next.take(); // 需要删除的开始节点
        // p2 移到待删除的最后一个节点
        let mut p2 = del_head.as_deref_mut().unwrap();
        for _ in a..b {
            p2 = p2.next.as_deref_mut().unwrap();
        }
        let tail = p2.next.take();

        p1.next = list2;
        while p1.next.is_some() {
            // 遍历到list2末尾
            p1 = p1.next.as_deref_mut().unwrap();
        }

        // 拼接
        p1.next = tail;

        list1
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
