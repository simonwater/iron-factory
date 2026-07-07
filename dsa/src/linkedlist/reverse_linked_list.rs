use crate::linkedlist::ListNode;
/// [206. 反转链表](https://leetcode.cn/problems/reverse-linked-list/description/)
pub struct Solution;

impl Solution {
    /// [92. 反转链表 II](https://leetcode.cn/problems/reverse-linked-list-ii/description/)
    pub fn reverse_between(
        head: Option<Box<ListNode>>,
        left: i32,
        right: i32,
    ) -> Option<Box<ListNode>> {
        if left >= right {
            return head;
        }
        // 分割成三部分：l1，l2，l3
        let mut dummy = ListNode::new(0);
        dummy.next = head;
        let mut l1_tail = &mut dummy;
        for _ in 1..left {
            l1_tail = l1_tail.next.as_deref_mut().unwrap();
        }

        // 取得l2后开始同步翻转，翻转完成后prev指向第2个链表头节点，l2指向第三个链表头节点
        let mut prev = None;
        let mut l2 = l1_tail.next.take();
        for _ in left..=right {
            if let Some(mut node) = l2 {
                let next = node.next.take();
                node.next = prev;
                prev = Some(node);
                l2 = next;
            }
        }

        let mut l2_tail = prev.as_mut().unwrap();
        while l2_tail.next.is_some() {
            l2_tail = l2_tail.next.as_mut().unwrap();
        }
        l2_tail.next = l2;
        l1_tail.next = prev;
        dummy.next
    }

    pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev: Option<Box<ListNode>> = None;
        while let Some(mut node) = head {
            head = node.next.take();
            node.next = prev;
            prev = Some(node);
        }
        prev
    }

    pub fn reverse_list_recursive(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::recursive(None, head)
    }

    fn recursive(prev: Option<Box<ListNode>>, cur: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match cur {
            None => prev,
            Some(mut node) => {
                let next = node.next.take();
                node.next = prev;
                Self::recursive(Some(node), next)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let l = ListNode::from_vec(vec![1, 2, 3, 4, 5]);
        let ll = ListNode::from_vec(vec![5, 4, 3, 2, 1]);
        assert_eq!(Solution::reverse_list(l), ll);

        let l = ListNode::from_vec(vec![1, 2]);
        let ll = ListNode::from_vec(vec![2, 1]);
        assert_eq!(Solution::reverse_list(l), ll);

        let l = ListNode::from_vec(vec![]);
        let ll = ListNode::from_vec(vec![]);
        assert_eq!(Solution::reverse_list(l), ll);
    }

    #[test]
    fn test2() {
        let l = ListNode::from_vec(vec![1, 2, 3, 4, 5]);
        let ll = ListNode::from_vec(vec![5, 4, 3, 2, 1]);
        assert_eq!(Solution::reverse_list_recursive(l), ll);

        let l = ListNode::from_vec(vec![1, 2]);
        let ll = ListNode::from_vec(vec![2, 1]);
        assert_eq!(Solution::reverse_list_recursive(l), ll);

        let l = ListNode::from_vec(vec![]);
        let ll = ListNode::from_vec(vec![]);
        assert_eq!(Solution::reverse_list_recursive(l), ll);
    }

    #[test]
    fn test3() {
        let l = ListNode::from_vec(vec![1, 2, 3, 4, 5]);
        let ll = ListNode::from_vec(vec![1, 4, 3, 2, 5]);
        assert_eq!(Solution::reverse_between(l, 2, 4), ll);

        let l = ListNode::from_vec(vec![5]);
        let ll = ListNode::from_vec(vec![5]);
        assert_eq!(Solution::reverse_between(l, 1, 1), ll);

        let l = ListNode::from_vec(vec![1, 2, 3, 4, 5]);
        let ll = ListNode::from_vec(vec![4, 3, 2, 1, 5]);
        assert_eq!(Solution::reverse_between(l, 1, 4), ll);

        let l = ListNode::from_vec(vec![1, 2, 3, 4, 5]);
        let ll = ListNode::from_vec(vec![5, 4, 3, 2, 1]);
        assert_eq!(Solution::reverse_between(l, 1, 5), ll);
    }
}
