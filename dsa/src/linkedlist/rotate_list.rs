use crate::linkedlist::ListNode;
/// [61. 旋转链表](https://leetcode.cn/problems/rotate-list/)
pub struct Solution;

impl Solution {
    pub fn rotate_right(mut head: Option<Box<ListNode>>, mut k: i32) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }
        // 计算斩断位置
        let total = Self::node_count(&head);
        k = k % total;
        if k == 0 {
            return head;
        }

        // 斩断链表
        let step = total - k - 1;
        let mut p = head.as_deref_mut().unwrap();
        for _ in 0..step {
            p = p.next.as_deref_mut().unwrap();
        }
        let mut new_head = p.next.take();

        // 重新拼接链表
        let mut tail = new_head.as_deref_mut().unwrap();
        while tail.next.is_some() {
            tail = tail.next.as_deref_mut().unwrap();
        }
        // while let Some(node) = tail.next.as_deref_mut() {  // 报错逻辑
        //     tail = node;
        // }
        tail.next = head;
        new_head
    }

    fn node_count(head: &Option<Box<ListNode>>) -> i32 {
        let mut total = 0;
        let mut p = head.as_deref();
        while let Some(node) = p {
            total += 1;
            p = node.next.as_deref();
        }
        total
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
