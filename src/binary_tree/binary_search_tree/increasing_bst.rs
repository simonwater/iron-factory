/// [897. 递增顺序搜索树](https://leetcode.cn/problems/increasing-order-search-tree/)
///
use crate::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

impl Solution {
    //
    pub fn increasing_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut ans = Rc::new(RefCell::new(TreeNode::new(0)));
        let mut prev = ans.clone();
        Self::dfs(root, &mut prev);
        ans.borrow_mut().right.take()
    }

    fn dfs(mut root: Option<Rc<RefCell<TreeNode>>>, prev: &mut Rc<RefCell<TreeNode>>) {
        if let Some(root_rc) = root {
            let (left, right) = {
                let mut root_node = root_rc.borrow_mut();
                (root_node.left.take(), root_node.right.take())
            };
            Self::dfs(left, prev);
            prev.borrow_mut().right = Some(Rc::clone(&root_rc));
            *prev = root_rc;
            Self::dfs(right, prev);
        }
    }
}

struct Solution2;

impl Solution2 {
    //
    pub fn increasing_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut ans = None;
        Self::dfs(root, &mut ans);
        ans
    }

    fn dfs(mut root: Option<Rc<RefCell<TreeNode>>>, ans: &mut Option<Rc<RefCell<TreeNode>>>) {
        if let Some(root_rc) = root {
            let (left, right) = {
                let mut root_node = root_rc.borrow_mut();
                (root_node.left.take(), root_node.right.take())
            };
            Self::dfs(right, ans);
            root_rc.borrow_mut().right = ans.clone();
            *ans = Some(root_rc);
            Self::dfs(left, ans);
        }
    }
}

struct Solution3;

impl Solution3 {
    //
    pub fn increasing_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let dumy = Rc::new(RefCell::new(TreeNode::new(0)));
        let mut prev = dumy.clone();
        let mut cur = root;
        while let Some(cur_rc) = cur {
            let left = {
                let mut cur_node = cur_rc.borrow_mut();
                cur_node.left.take()
            };
            if let Some(left_rc) = left {
                // 有左子树，把自己挂到左子树末尾，然后cur移到左子树
                let mut p = left_rc.clone();
                while p.borrow().right.is_some() {
                    let next = p.borrow().right.clone();
                    p = next.unwrap();
                }
                p.borrow_mut().right = Some(cur_rc);
                cur = Some(left_rc);
            } else {
                // 没有左子树，cur往右移
                prev.borrow_mut().right = Some(Rc::clone(&cur_rc));
                prev = cur_rc;
                cur = prev.borrow().right.clone();
            }
        }

        dumy.borrow_mut().right.take()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree;

    #[test]
    fn test1() {}
}
