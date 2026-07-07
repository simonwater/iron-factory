use crate::linkedlist::ListNode;

/// [141. 环形链表](https://leetcode.cn/problems/linked-list-cycle/description/)
pub struct Solution;

impl Solution {
    pub fn has_cycle(head: Option<Box<ListNode>>) -> bool {
        let mut slow = head.as_ref();
        let mut fast = head.as_ref();
        while let (Some(s), Some(f)) = (slow, fast) {
            slow = s.next.as_ref();
            fast = f.next.as_ref();
            if let Some(f_next) = fast {
                fast = f_next.next.as_ref();
            } else {
                return false;
            }

            if let (Some(s_ref), Some(f_ref)) = (slow, fast) {
                if std::ptr::eq(s_ref, f_ref) {
                    return true;
                }
            }
        }

        false
    }

    pub fn has_cycle2(head: Option<Box<ListNode>>) -> bool {
        let mut slow = head.as_deref();
        let mut fast = head.as_deref();
        while let (Some(s), Some(f)) = (slow, fast) {
            slow = s.next.as_deref();
            fast = f.next.as_deref();
            if let Some(f_next) = fast {
                fast = f_next.next.as_deref();
            } else {
                return false;
            }

            if let (Some(s_ref), Some(f_ref)) = (slow, fast) {
                if std::ptr::eq(s_ref, f_ref) {
                    return true;
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let l = ListNode::from_vec(vec![]);
        assert!(!Solution::has_cycle(l));

        let l = ListNode::from_vec(vec![1, 2, 3]);
        assert!(!Solution::has_cycle(l));
    }

    #[test]
    fn test2() {
        let l = ListNode::from_vec(vec![]);
        assert!(!Solution::has_cycle2(l));

        let l = ListNode::from_vec(vec![1, 2, 3]);
        assert!(!Solution::has_cycle2(l));
    }
}
