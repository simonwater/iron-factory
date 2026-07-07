use crate::linkedlist::ListNode;

/// [876. 链表的中间结点](https://leetcode.cn/problems/middle-of-the-linked-list/description/)
pub struct Solution;

impl Solution {
    pub fn middle_node(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut slow_cnt = 0;
        let mut fast = head.as_deref();
        while let Some(f) = fast {
            if let Some(f_next) = f.next.as_deref() {
                fast = f_next.next.as_deref();
                slow_cnt += 1;
            } else {
                break;
            }
        }

        for _ in 0..slow_cnt {
            head = head.unwrap().next;
        }
        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let l1 = ListNode::from_vec(vec![]);
        let l2 = ListNode::from_vec(vec![]);
        assert_eq!(Solution::middle_node(l1), l2);

        let l1 = ListNode::from_vec(vec![1]);
        let l2 = ListNode::from_vec(vec![1]);
        assert_eq!(Solution::middle_node(l1), l2);

        let l1 = ListNode::from_vec(vec![1, 2, 3, 4, 5]);
        let l2 = ListNode::from_vec(vec![3, 4, 5]);
        assert_eq!(Solution::middle_node(l1), l2);

        let l1 = ListNode::from_vec(vec![1, 2, 3, 4, 5, 6]);
        let l2 = ListNode::from_vec(vec![4, 5, 6]);
        assert_eq!(Solution::middle_node(l1), l2);
    }
}
