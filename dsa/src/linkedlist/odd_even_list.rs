/// [328. 奇偶链表](https://leetcode.cn/problems/odd-even-linked-list/description/)
use crate::linkedlist::ListNode;

pub struct Solution;

impl Solution {
    pub fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut l1, mut l2) = (ListNode::new(0), ListNode::new(0));
        let (mut p1, mut p2) = (&mut l1, &mut l2);
        let mut cur = head;
        let mut flag = true;
        while let Some(mut node) = cur {
            let next = node.next.take();
            if flag {
                p1.next = Some(node);
                p1 = p1.next.as_mut().unwrap();
            } else {
                p2.next = Some(node);
                p2 = p2.next.as_mut().unwrap();
            }
            flag = !flag;
            cur = next;
        }
        p1.next = l2.next;
        l1.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let l = ListNode::from_vec(vec![1, 2, 3, 4, 5]);
        let ll = ListNode::from_vec(vec![1, 3, 5, 2, 4]);
        assert_eq!(Solution::odd_even_list(l), ll);

        let l = ListNode::from_vec(vec![2, 1, 3, 5, 6, 4, 7]);
        let ll = ListNode::from_vec(vec![2, 3, 6, 7, 1, 5, 4]);
        assert_eq!(Solution::odd_even_list(l), ll);
    }
}
