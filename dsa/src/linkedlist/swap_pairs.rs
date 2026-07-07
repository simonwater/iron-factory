/// [24. 两两交换链表中的节点](https://leetcode.cn/problems/swap-nodes-in-pairs/description/)
use crate::linkedlist::ListNode;
pub struct Solution;

impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }
        let mut dumy = ListNode::new(0);
        dumy.next = head;
        let mut prev = &mut dumy;
        while prev.next.is_some() && prev.next.as_ref().unwrap().next.is_some() {
            let mut first_node = prev.next.take().unwrap();
            let mut second_node = first_node.next.take().unwrap();
            let third = second_node.next.take();

            first_node.next = third;
            second_node.next = Some(first_node);
            prev.next = Some(second_node);

            prev = prev
                .next
                .as_deref_mut()
                .unwrap()
                .next
                .as_deref_mut()
                .unwrap();
        }

        dumy.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let l = ListNode::from_vec(vec![]);
        let ll = ListNode::from_vec(vec![]);
        assert_eq!(Solution::swap_pairs(l), ll);

        let l = ListNode::from_vec(vec![1]);
        let ll = ListNode::from_vec(vec![1]);
        assert_eq!(Solution::swap_pairs(l), ll);

        let l = ListNode::from_vec(vec![1, 2, 3, 4]);
        let ll = ListNode::from_vec(vec![2, 1, 4, 3]);
        assert_eq!(Solution::swap_pairs(l), ll);
    }
}
