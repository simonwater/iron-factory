/// [2. 两数相加](https://leetcode.cn/problems/add-two-numbers/description/)
use crate::linkedlist::ListNode;

pub struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        Self::add_two(l1, l2, 0)
    }

    fn add_two(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
        cary: i32,
    ) -> Option<Box<ListNode>> {
        if l1.is_none() && l2.is_none() && cary == 0 {
            return None;
        }
        let mut sum = cary;
        let mut next1 = None;
        let mut next2 = None;

        if let Some(node1) = l1 {
            sum += node1.val;
            next1 = node1.next;
        }
        if let Some(node2) = l2 {
            sum += node2.val;
            next2 = node2.next;
        }
        Some(Box::new(ListNode {
            val: sum % 10,
            next: Self::add_two(next1, next2, sum / 10),
        }))
    }

    pub fn add_two_numbers_iter(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dumy = ListNode::new(0);
        let mut cur = &mut dumy;
        let mut cary = 0;
        while l1.is_some() || l2.is_some() || cary != 0 {
            if let Some(node) = l1 {
                cary += node.val;
                l1 = node.next;
            }
            if let Some(node) = l2 {
                cary += node.val;
                l2 = node.next;
            }
            cur.next = Some(Box::new(ListNode::new(cary % 10)));
            cur = cur.next.as_deref_mut().unwrap();
            cary = cary / 10;
        }
        dumy.next
    }

    pub fn add_two_numbers_bigendian(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut stack1 = Vec::new();
        let mut stack2 = Vec::new();
        while let Some(node) = l1 {
            stack1.push(node.val);
            l1 = node.next;
        }
        while let Some(node) = l2 {
            stack2.push(node.val);
            l2 = node.next;
        }

        let mut cary = 0;
        let mut head = None;
        while !stack1.is_empty() || !stack2.is_empty() || cary != 0 {
            cary += stack1.pop().unwrap_or(0);
            cary += stack2.pop().unwrap_or(0);
            let mut node = ListNode::new(cary % 10);
            node.next = head;
            head = Some(Box::new(node));
            cary /= 10;
        }

        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let l1 = ListNode::from_vec(vec![2, 4, 3]);
        let l2 = ListNode::from_vec(vec![5, 6, 4]);
        let l3 = ListNode::from_vec(vec![7, 0, 8]);
        assert_eq!(Solution::add_two_numbers(l1, l2), l3);

        let l1 = ListNode::from_vec(vec![0]);
        let l2 = ListNode::from_vec(vec![0]);
        let l3 = ListNode::from_vec(vec![0]);
        assert_eq!(Solution::add_two_numbers(l1, l2), l3);

        let l1 = ListNode::from_vec(vec![9, 9, 9, 9, 9, 9, 9]);
        let l2 = ListNode::from_vec(vec![9, 9, 9, 9]);
        let l3 = ListNode::from_vec(vec![8, 9, 9, 9, 0, 0, 0, 1]);
        assert_eq!(Solution::add_two_numbers(l1, l2), l3);
    }

    #[test]
    fn test2() {
        let l1 = ListNode::from_vec(vec![2, 4, 3]);
        let l2 = ListNode::from_vec(vec![5, 6, 4]);
        let l3 = ListNode::from_vec(vec![7, 0, 8]);
        assert_eq!(Solution::add_two_numbers_iter(l1, l2), l3);

        let l1 = ListNode::from_vec(vec![0]);
        let l2 = ListNode::from_vec(vec![0]);
        let l3 = ListNode::from_vec(vec![0]);
        assert_eq!(Solution::add_two_numbers_iter(l1, l2), l3);

        let l1 = ListNode::from_vec(vec![9, 9, 9, 9, 9, 9, 9]);
        let l2 = ListNode::from_vec(vec![9, 9, 9, 9]);
        let l3 = ListNode::from_vec(vec![8, 9, 9, 9, 0, 0, 0, 1]);
        assert_eq!(Solution::add_two_numbers_iter(l1, l2), l3);
    }

    #[test]
    fn test3() {
        let l1 = ListNode::from_vec(vec![7, 2, 4, 3]);
        let l2 = ListNode::from_vec(vec![5, 6, 4]);
        let l3 = ListNode::from_vec(vec![7, 8, 0, 7]);
        assert_eq!(Solution::add_two_numbers_bigendian(l1, l2), l3);

        let l1 = ListNode::from_vec(vec![2, 4, 3]);
        let l2 = ListNode::from_vec(vec![5, 6, 4]);
        let l3 = ListNode::from_vec(vec![8, 0, 7]);
        assert_eq!(Solution::add_two_numbers_bigendian(l1, l2), l3);

        let l1 = ListNode::from_vec(vec![0]);
        let l2 = ListNode::from_vec(vec![0]);
        let l3 = ListNode::from_vec(vec![0]);
        assert_eq!(Solution::add_two_numbers_bigendian(l1, l2), l3);
    }
}
