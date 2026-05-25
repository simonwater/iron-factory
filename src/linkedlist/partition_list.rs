use crate::linkedlist::ListNode;
/// [86. 分隔链表](https://leetcode.cn/problems/partition-list/)
struct Solution;

impl Solution {
    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut dummy1 = ListNode::new(0);
        let mut dummy2 = ListNode::new(0);
        let mut p1 = &mut dummy1;
        let mut p2 = &mut dummy2;
        let mut cur = head;
        while let Some(mut node) = cur {
            let next = node.next.take(); // 先把下一个节点取走暂存起来
            if node.val < x {
                p1.next = Some(node);
                p1 = p1.next.as_mut().unwrap();
            } else {
                p2.next = Some(node);
                p2 = p2.next.as_mut().unwrap();
            }
            cur = next;
        }
        p1.next = dummy2.next;
        dummy1.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let l = ListNode::from_vec(vec![1, 4, 3, 2, 5, 2]);
        let x = 3;
        let ans = ListNode::from_vec(vec![1, 2, 2, 4, 3, 5]);
        assert_eq!(Solution::partition(l, x), ans);

        let l = ListNode::from_vec(vec![2, 1]);
        let x = 2;
        let ans = ListNode::from_vec(vec![1, 2]);
        assert_eq!(Solution::partition(l, x), ans);

        let l = ListNode::from_vec(vec![]);
        let x = 2;
        let ans = ListNode::from_vec(vec![]);
        assert_eq!(Solution::partition(l, x), ans);
    }
}
