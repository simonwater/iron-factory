use crate::linkedlist::ListNode;
/// [25. K 个一组翻转链表](https://leetcode.cn/problems/reverse-nodes-in-k-group/)
pub struct Solution;

/// “探测 -> 斩断 -> 反转 -> 缝合 -> 推进”
impl Solution {
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if head.is_none() || head.as_ref().unwrap().next.is_none() || k <= 1 {
            return head;
        }
        let mut dummy = ListNode::new(0);
        dummy.next = head;
        let mut prev_list_tail = &mut dummy;
        while prev_list_tail.next.is_some() {
            // 1. 探测：找到k个
            let mut p = prev_list_tail.next.as_deref_mut().unwrap();
            for _ in 0..k - 1 {
                if p.next.is_none() {
                    p.val = -1;
                    return dummy.next; // 不够k个
                }
                p = p.next.as_deref_mut().unwrap();
            }
            // 2. 斩断：暂存下一段的头节点
            let next_list_head = p.next.take();

            // 3.反转：当前k个节点的链表
            prev_list_tail.next = Self::reverse_list(prev_list_tail.next.take());

            // 4.缝合：找到尾部，拼接回下一段
            let mut cur_tail = prev_list_tail.next.as_deref_mut().unwrap();
            while cur_tail.next.is_some() {
                cur_tail = cur_tail.next.as_deref_mut().unwrap();
            }
            cur_tail.next = next_list_head;
            // 5.推进：开始准备处理下一段
            prev_list_tail = cur_tail;
        }
        dummy.next
    }

    fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev = None;
        while let Some(mut node) = head {
            let next = node.next.take();
            node.next = prev;
            prev = Some(node);
            head = next;
        }
        prev
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
