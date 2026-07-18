//! [83. 删除排序链表中的重复元素](https://leetcode.cn/problems/remove-duplicates-from-sorted-list/)
//!
use crate::linkedlist::ListNode;

pub struct Solution;
impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_some() {
            let mut cur = head.as_deref_mut().unwrap();
            while cur.next.is_some() {
                if cur.val == cur.next.as_ref().unwrap().val {
                    let next = cur.next.as_deref_mut().unwrap();
                    cur.next = next.next.take();
                } else {
                    cur = cur.next.as_deref_mut().unwrap();
                }
            }
        }
        head
    }
}

///
pub struct Solution2;
impl Solution2 {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut cur = head.as_deref_mut();
        while let Some(node) = cur {
            if let Some(next) = node.next.as_deref_mut() {
                // cur内部的所有权已经被移给node，所以两个分支都要重新赋值。否则下一轮循环继续使用的cur就是无效的
                if node.val == next.val {
                    let nexnext = next.next.take(); // next 生命周期到此结束，后面可正常对node.next赋值
                    node.next = nexnext;
                    cur = Some(node);
                } else {
                    cur = node.next.as_deref_mut();
                }
            } else {
                // cur内部的所有权虽然被移给node，但break以后 cur 不再被使用，处于无效状态完全合规
                break;
            }
        }
        head
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test1() {
        // 这里的借用检查规则等价于Solution2中对链表节点的操作
        let mut a = 123;
        let b = &a;
        let c = (*b) * 2;
        a = c; // b生命周期在上一行已经结束，可以安全赋值
        // print!("{b}"); // 这里如果还用到b，则与规则“只能存在一个可变借用”冲突
        println!("{a}");
    }
}
