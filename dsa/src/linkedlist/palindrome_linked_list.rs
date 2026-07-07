use crate::linkedlist::ListNode;

/// [234. 回文链表](https://leetcode.cn/problems/palindrome-linked-list/description/)
pub struct Solution;

impl Solution {
    pub fn is_palindrome(mut head: Option<Box<ListNode>>) -> bool {
        if head.is_none() || head.as_deref().unwrap().next.is_none() {
            return true;
        }
        // 定位中点
        let mut slow_index = 0;
        let mut fast = head.as_deref().unwrap().next.as_deref();
        while let Some(f) = fast {
            if let Some(f_next) = f.next.as_deref() {
                fast = f_next.next.as_deref();
                slow_index += 1;
            } else {
                break;
            }
        }

        // 获取后半部开头
        let mut head2 = {
            let mut p = head.as_mut().unwrap();
            for _ in 0..slow_index {
                p = p.next.as_mut().unwrap();
            }
            p.next.take()
        };

        // 翻转
        let mut prev: Option<Box<ListNode>> = None;
        while let Some(mut node) = head2 {
            let next = node.next.take();
            node.next = prev;
            prev = Some(node);
            head2 = next;
        }

        // 比较
        let mut p1 = head.as_deref();
        let mut p2 = prev.as_deref();
        while let (Some(node1), Some(node2)) = (p1, p2) {
            if node1.val != node2.val {
                return false;
            }
            p1 = node1.next.as_deref();
            p2 = node2.next.as_deref();
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let l = ListNode::from_vec(vec![1, 2, 2, 1]);
        assert_eq!(Solution::is_palindrome(l), true);

        let l = ListNode::from_vec(vec![1, 2]);
        assert_eq!(Solution::is_palindrome(l), false);

        let l = ListNode::from_vec(vec![1, 2, 3, 2, 1]);
        assert_eq!(Solution::is_palindrome(l), true);
    }
}
