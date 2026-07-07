/// [148. 排序链表](https://leetcode.cn/problems/sort-list/description/)
use crate::linkedlist::ListNode;
pub struct Solution;

impl Solution {
    pub fn sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            return head;
        }

        // 1. 找中点
        let mut slow_steps = 0;
        let mut fast = head.as_deref();
        while let Some(f) = fast {
            if let Some(f_next) = f.next.as_deref() {
                slow_steps += 1;
                fast = f_next.next.as_deref();
            } else {
                break;
            }
        }
        let mut middle = &mut head;
        for _ in 0..slow_steps {
            middle = &mut middle.as_mut().unwrap().next;
        }
        let mut head2 = middle.take();

        // 2. 排序子链表
        head = Self::sort_list(head);
        head2 = Self::sort_list(head2);

        // 3. 合并
        Self::merge(head, head2)
    }

    fn merge(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dumy = ListNode::new(0);
        let mut cur = &mut dumy;
        while l1.is_some() && l2.is_some() {
            let val1 = l1.as_ref().unwrap().val;
            let val2 = l2.as_ref().unwrap().val;
            if val1 <= val2 {
                let next = l1.as_mut().unwrap().next.take();
                cur.next = l1;
                l1 = next;
            } else {
                let next = l2.as_mut().unwrap().next.take();
                cur.next = l2;
                l2 = next;
            }
            cur = cur.next.as_deref_mut().unwrap();
        }
        cur.next = if l1.is_none() { l2 } else { l1 };
        dumy.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let l = ListNode::from_vec(vec![4, 2, 1, 3]);
        let ll = ListNode::from_vec(vec![1, 2, 3, 4]);
        assert_eq!(Solution::sort_list(l), ll);

        let l = ListNode::from_vec(vec![-1, 5, 3, 4, 0]);
        let ll = ListNode::from_vec(vec![-1, 0, 3, 4, 5]);
        assert_eq!(Solution::sort_list(l), ll);

        let l = ListNode::from_vec(vec![]);
        let ll = ListNode::from_vec(vec![]);
        assert_eq!(Solution::sort_list(l), ll);
    }
}
