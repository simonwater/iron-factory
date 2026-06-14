/// # [198. 打家劫舍](https://leetcode.cn/problems/house-robber/)
///
use crate::binary_tree::TreeNode;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
struct Solution;

impl Solution {
    // 自顶向下记忆化搜索
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut memo = HashMap::with_capacity(32);
        Self::dfs(&root, &mut memo)
    }

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, memo: &mut HashMap<usize, i32>) -> i32 {
        if let Some(root_rc) = root {
            let key = root_rc.as_ptr() as usize;
            if let Some(&val) = memo.get(&key) {
                return val;
            }

            let node = root_rc.borrow();
            // 不选当前节点，直接取两个左右子树的和
            let mut ans1 = Self::dfs(&node.left, memo) + Self::dfs(&node.right, memo);
            // 取当前节点，再加上当前节点的孙节点
            let mut ans2 = node.val;
            if let Some(left_rc) = &node.left {
                let left_node = left_rc.borrow();
                ans2 += Self::dfs(&left_node.left, memo) + Self::dfs(&left_node.right, memo);
            }
            if let Some(right_rc) = &node.right {
                let right_node = right_rc.borrow();
                ans2 += Self::dfs(&right_node.left, memo) + Self::dfs(&right_node.right, memo);
            }
            if ans2 > ans1 {
                ans1 = ans2;
            }
            memo.insert(key, ans1);
            ans1
        } else {
            0
        }
    }
}

struct Solution2;
impl Solution2 {
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let ans = Self::dfs(&root);
        i32::max(ans.0, ans.1)
    }

    // (选择, 不选择)
    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        if let Some(root_rc) = root {
            let node = root_rc.borrow();
            let (rob_l, not_rob_l) = Self::dfs(&node.left);
            let (rob_r, not_rob_r) = Self::dfs(&node.right);
            let rob_cur = node.val + not_rob_l + not_rob_r;
            let not_rob_cur = i32::max(rob_l, not_rob_l) + i32::max(rob_r, not_rob_r);
            (rob_cur, not_rob_cur)
        } else {
            (0, 0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
