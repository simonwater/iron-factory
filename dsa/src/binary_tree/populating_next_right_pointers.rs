use crate::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

/// [116. 填充每个节点的下一个右侧节点指针](https://leetcode.cn/problems/populating-next-right-pointers-in-each-node/)
pub struct Solution;
// 满二叉树
impl Solution {
    pub fn connect(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        root
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
