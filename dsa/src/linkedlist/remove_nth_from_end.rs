/// [19. 删除链表的倒数第 N 个结点](https://leetcode.cn/problems/remove-nth-node-from-end-of-list/description/)
use crate::linkedlist::ListNode;

pub struct Solution;

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dumy = ListNode::new(0);
        dumy.next = head;
        let mut cur = dumy.next.as_deref();
        let mut sz = 0;
        while let Some(node) = cur {
            sz += 1;
            cur = node.next.as_deref();
        }

        let step = sz - n;
        let mut prev = &mut dumy;
        for _ in 0..step {
            prev = prev.next.as_mut().unwrap();
        }

        let next = prev.next.take();
        if let Some(mut next_node) = next {
            prev.next = next_node.next.take();
        }
        dumy.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let l = ListNode::from_vec(vec![1, 2, 3, 4, 5]);
        let ll = ListNode::from_vec(vec![1, 2, 3, 5]);
        assert_eq!(Solution::remove_nth_from_end(l, 2), ll);

        let l = ListNode::from_vec(vec![1]);
        let ll = ListNode::from_vec(vec![]);
        assert_eq!(Solution::remove_nth_from_end(l, 1), ll);

        let l = ListNode::from_vec(vec![1, 2]);
        let ll = ListNode::from_vec(vec![1]);
        assert_eq!(Solution::remove_nth_from_end(l, 1), ll);
    }
}
