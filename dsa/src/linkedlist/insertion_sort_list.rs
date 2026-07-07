use crate::linkedlist::ListNode;
/// [147. 对链表进行插入排序](https://leetcode.cn/problems/insertion-sort-list/)
pub struct Solution;

impl Solution {
    pub fn insertion_sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(i32::MIN);
        while let Some(mut node) = head {
            let new_unorder = node.next.take();
            let mut p = &mut dummy;
            while p.next.is_some() && node.val >= p.next.as_ref().unwrap().val {
                p = p.next.as_deref_mut().unwrap();
            }
            node.next = p.next.take();
            p.next = Some(node);
            head = new_unorder;
        }

        dummy.next
    }
}

// 已经有序时进行优化，需要用到裸指针
pub struct Solution2;
impl Solution2 {
    pub fn insertion_sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(i32::MIN);

        let mut last_sorted_ptr: *mut ListNode = &mut dummy;
        while let Some(mut node) = head {
            let new_unorder = node.next.take(); // 当前节点后面的斩断

            let last_val = unsafe { (*last_sorted_ptr).val };
            if last_val <= node.val {
                unsafe {
                    (*last_sorted_ptr).next = Some(node);
                    last_sorted_ptr = (*last_sorted_ptr).next.as_deref_mut().unwrap();
                }
            } else {
                let mut p = &mut dummy;
                while p.next.is_some() && node.val >= p.next.as_ref().unwrap().val {
                    p = p.next.as_deref_mut().unwrap();
                }
                node.next = p.next.take();
                p.next = Some(node);
            }

            head = new_unorder;
        }

        dummy.next
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
