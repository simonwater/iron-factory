use crate::linkedlist::ListNode;
/// [23. 合并 K 个升序链表](https://leetcode.cn/problems/merge-k-sorted-lists/description/)
pub struct Solution;

impl Solution {
    pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        Self::merge(&mut lists)
    }

    fn merge(lists: &mut [Option<Box<ListNode>>]) -> Option<Box<ListNode>> {
        let n = lists.len();
        if n == 0 {
            return None;
        }
        if n == 1 {
            return lists[0].take();
        }
        let mid = n / 2;
        let (left, right) = lists.split_at_mut(mid);
        let l1 = Self::merge(left);
        let l2 = Self::merge(right);

        Self::merge_two_lists(l1, l2)
    }

    fn merge_two_lists(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        let mut l = &mut dummy;
        while l1.is_some() && l2.is_some() {
            let val1 = l1.as_ref().unwrap().val;
            let val2 = l2.as_ref().unwrap().val;
            if val1 <= val2 {
                let next = l1.as_mut().unwrap().next.take();
                l.next = l1;
                l1 = next;
                l = l.next.as_deref_mut().unwrap();
            } else {
                let next = l2.as_mut().unwrap().next.take();
                l.next = l2;
                l2 = next;
                l = l.next.as_deref_mut().unwrap();
            }
        }
        l.next = l1.or(l2);
        dummy.next.take()
    }
}

pub struct Solution2;

use std::{cmp::Ordering, collections::BinaryHeap};
#[derive(Eq, PartialEq)]
struct MyNode(Box<ListNode>);
impl Ord for MyNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.val.cmp(&self.0.val)
    }
}
impl PartialOrd for MyNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution2 {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        if lists.is_empty() {
            return None;
        }
        let mut heap = BinaryHeap::with_capacity(lists.len());
        for list in lists {
            if let Some(node) = list {
                heap.push(MyNode(node));
            }
        }
        let mut dummy = ListNode::new(0);
        let mut l = &mut dummy;
        while let Some(MyNode(mut list)) = heap.pop() {
            if let Some(next) = list.next.take() {
                heap.push(MyNode(next));
            }
            l.next = Some(list);
            l = l.next.as_deref_mut().unwrap();
        }

        dummy.next
    }
}
