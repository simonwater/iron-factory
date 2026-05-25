use crate::linkedlist::ListNode;
/// [143. 重排链表](https://leetcode.cn/problems/reorder-list/description/)
struct Solution;

impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        if head.is_none() {
            return;
        }
        // 分离出后半段
        let mut slow_cnt = 0;
        let mut fast = head.as_deref().unwrap().next.as_deref();
        while let Some(node) = fast {
            if let Some(f_next) = node.next.as_deref() {
                fast = f_next.next.as_deref();
                slow_cnt += 1;
            } else {
                break;
            }
        }
        let mut p = head.as_deref_mut().unwrap();
        for _ in 0..slow_cnt {
            p = p.next.as_deref_mut().unwrap();
        }
        let mut l2 = p.next.take();

        // 反转l2
        let mut prev = None;
        while let Some(mut node) = l2 {
            let next = node.next.take();
            node.next = prev;
            prev = Some(node);
            l2 = next;
        }

        // 合并
        // head始终不被改变，避免“所有权真空期”
        let mut l1 = head.as_mut().unwrap().next.take();
        let mut l2 = prev;
        let mut l = head.as_deref_mut().unwrap();
        while l1.is_some() && l2.is_some() {
            let mut node1 = l1.take().unwrap();
            let mut node2 = l2.take().unwrap();

            let l1_next = node1.next.take();
            let l2_next = node2.next.take();
            node2.next = Some(node1);
            l.next = Some(node2);
            l = l.next.as_deref_mut().unwrap().next.as_deref_mut().unwrap();
            l1 = l1_next;
            l2 = l2_next;
        }
        l.next = if l1.is_some() { l1 } else { l2 };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut l = ListNode::from_vec(vec![1, 2, 3, 4]);
        let ans = ListNode::from_vec(vec![1, 4, 2, 3]);
        Solution::reorder_list(&mut l);
        assert_eq!(l, ans);

        let mut l = ListNode::from_vec(vec![1, 2, 3, 4, 5]);
        let ans = ListNode::from_vec(vec![1, 5, 2, 4, 3]);
        Solution::reorder_list(&mut l);
        assert_eq!(l, ans);
    }
}
