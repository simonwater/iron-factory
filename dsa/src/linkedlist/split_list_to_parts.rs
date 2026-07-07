use crate::linkedlist::ListNode;
/// [725. 分隔链表](https://leetcode.cn/problems/split-linked-list-in-parts/)
pub struct Solution;

impl Solution {
    // 把链表分为 k 组
    pub fn split_list_to_parts(
        mut head: Option<Box<ListNode>>,
        k: i32,
    ) -> Vec<Option<Box<ListNode>>> {
        let mut cnt = 0;
        let mut p = head.as_deref();
        while let Some(node) = p {
            cnt += 1;
            p = node.next.as_deref();
        }

        let k = k as usize;
        let n1 = cnt / k; // 每组个数为n1
        let n2 = cnt % k; // 给前n2组再加一个节点

        let mut ans = vec![None; k];
        for i in 0..k {
            // 当前组的节点数
            let cur_cnt = n1 + if i < n2 { 1 } else { 0 };
            if cur_cnt == 0 {
                break;
            }
            let mut p = head.as_deref_mut().unwrap();
            for _ in 1..cur_cnt {
                p = p.next.as_deref_mut().unwrap();
            }
            let next = p.next.take();
            ans[i] = head;
            head = next;
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
