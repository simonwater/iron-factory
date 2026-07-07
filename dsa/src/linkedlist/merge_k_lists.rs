use crate::linkedlist::ListNode;
/// [23. 合并 K 个升序链表](https://leetcode.cn/problems/merge-k-sorted-lists/description/)
pub struct Solution;

impl Solution {
    pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        if lists.is_empty() {
            return None;
        }
        Self::merge(&mut lists)
    }

    fn merge(lists: &mut [Option<Box<ListNode>>]) -> Option<Box<ListNode>> {
        if lists.len() == 1 {
            return lists[0].take();
        }
        let mid = lists.len() / 2;
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
            if l1.as_ref().unwrap().val <= l2.as_ref().unwrap().val {
                let mut node = l1.unwrap();
                let next = node.next.take();
                l.next = Some(node);
                l1 = next;
            } else {
                let mut node = l2.unwrap();
                let next = node.next.take();
                l.next = Some(node);
                l2 = next;
            }
            l = l.next.as_deref_mut().unwrap();
        }
        l.next = l1.or(l2);
        dummy.next
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        let mut vec = vec![1];
        let mid = vec.len() / 2;
        let (l, r) = vec.split_at_mut(mid);
        println!("left len: {}", l.len());
        l.iter().for_each(|v| print!("{v} "));
        println!();

        println!("right len: {}", r.len());
        r.iter().for_each(|v| print!("{v} "));
        println!();
    }
}
