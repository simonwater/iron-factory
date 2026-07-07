use crate::linkedlist::ListNode;

/// [21. 合并两个有序链表](https://leetcode.cn/problems/merge-two-sorted-lists/description/)
pub struct Solution;

impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (None, None) => None,
            (Some(l1), None) => Some(l1),
            (None, Some(l2)) => Some(l2),
            (Some(mut l1), Some(mut l2)) => {
                if l1.val <= l2.val {
                    l1.next = Self::merge_two_lists(l1.next.take(), Some(l2));
                    Some(l1)
                } else {
                    l2.next = Self::merge_two_lists(Some(l1), l2.next.take());
                    Some(l2)
                }
            }
        }
    }

    pub fn merge_two_lists_iter(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dumy = ListNode::new(0);
        let mut cur = &mut dumy;
        while l1.is_some() && l2.is_some() {
            if l1.as_ref().unwrap().val <= l2.as_ref().unwrap().val {
                let mut node = l1.take().unwrap();
                l1 = node.next.take();
                cur.next = Some(node);
            } else {
                let mut node = l2.take().unwrap();
                l2 = node.next.take();
                cur.next = Some(node);
            }
            cur = cur.next.as_mut().unwrap();
        }
        cur.next = if l1.is_none() { l2 } else { l1 };
        dumy.next
    }

    pub fn merge_two_lists_iter1(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dumy = ListNode::new(0);
        let mut cur = &mut dumy;
        while l1.is_some() && l2.is_some() {
            if l1.as_ref().unwrap().val <= l2.as_ref().unwrap().val {
                let next = l1.as_mut().unwrap().next.take();
                cur.next = l1;
                l1 = next;
            } else {
                let next = l2.as_mut().unwrap().next.take();
                cur.next = l2;
                l2 = next;
            }
            cur = cur.next.as_mut().unwrap();
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
        let list1 = ListNode::from_vec(vec![1, 2, 4]);
        let list2 = ListNode::from_vec(vec![1, 3, 4]);
        let list3 = ListNode::from_vec(vec![1, 1, 2, 3, 4, 4]);
        assert_eq!(Solution::merge_two_lists(list1, list2), list3);

        let list1 = ListNode::from_vec(vec![]);
        let list2 = ListNode::from_vec(vec![]);
        let list3 = ListNode::from_vec(vec![]);
        assert_eq!(Solution::merge_two_lists(list1, list2), list3);

        let list1 = ListNode::from_vec(vec![]);
        let list2 = ListNode::from_vec(vec![0]);
        let list3 = ListNode::from_vec(vec![0]);
        assert_eq!(Solution::merge_two_lists(list1, list2), list3);
    }

    #[test]
    fn test2() {
        let list1 = ListNode::from_vec(vec![1, 2, 4]);
        let list2 = ListNode::from_vec(vec![1, 3, 4]);
        let list3 = ListNode::from_vec(vec![1, 1, 2, 3, 4, 4]);
        assert_eq!(Solution::merge_two_lists_iter(list1, list2), list3);

        let list1 = ListNode::from_vec(vec![]);
        let list2 = ListNode::from_vec(vec![]);
        let list3 = ListNode::from_vec(vec![]);
        assert_eq!(Solution::merge_two_lists_iter(list1, list2), list3);

        let list1 = ListNode::from_vec(vec![]);
        let list2 = ListNode::from_vec(vec![0]);
        let list3 = ListNode::from_vec(vec![0]);
        assert_eq!(Solution::merge_two_lists_iter(list1, list2), list3);
    }
}
