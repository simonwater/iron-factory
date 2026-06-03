use crate::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

/// [114. 二叉树展开为链表](https://leetcode.cn/problems/flatten-binary-tree-to-linked-list/)

// 1. 前序遍历
struct Solution;
impl Solution {
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut dummy = Rc::new(RefCell::new(TreeNode::new(0)));
        let mut prev = dummy.clone();
        Self::dfs(root.take(), &mut prev);
        *root = dummy.borrow_mut().right.take();
    }

    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, prev: &mut Rc<RefCell<TreeNode>>) {
        if let Some(node_rc) = root {
            let (left, right) = {
                let mut node = node_rc.borrow_mut();
                (node.left.take(), node.right.take())
            };
            prev.borrow_mut().right = Some(node_rc.clone());
            *prev = node_rc;
            Self::dfs(left, prev);
            Self::dfs(right, prev);
        }
    }
}

// 逆前序遍历
struct Solution2;
impl Solution2 {
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut ans: Option<Rc<RefCell<TreeNode>>> = None;
        Self::dfs(root.take(), &mut ans);
        *root = ans;
    }

    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, next: &mut Option<Rc<RefCell<TreeNode>>>) {
        if let Some(node_rc) = root {
            let (left, right) = {
                let mut node = node_rc.borrow_mut();
                (node.left.take(), node.right.take())
            };
            Self::dfs(right, next);
            Self::dfs(left, next);

            node_rc.borrow_mut().right = next.take();
            *next = Some(node_rc);
        }
    }
}

// 前序遍历迭代法
struct Solution3;
impl Solution3 {
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        if root.is_none() {
            return;
        }
        let mut dummy = Rc::new(RefCell::new(TreeNode::new(0)));
        let prev = &mut dummy;
        let mut stack = Vec::with_capacity(16);
        let root_rc = root.as_ref().unwrap();
        stack.push(Rc::clone(root_rc));
        while let Some(node_rc) = stack.pop() {
            let (left, right) = {
                let mut node = node_rc.borrow_mut();
                (node.left.take(), node.right.take())
            };
            prev.borrow_mut().right = Some(Rc::clone(&node_rc));
            *prev = node_rc;
            if let Some(right_rc) = right {
                stack.push(Rc::clone(&right_rc));
            }
            if let Some(left_rc) = left {
                stack.push(Rc::clone(&left_rc));
            }
        }
    }
}

/// O(1)空间复杂度的原地处理
/// 对树节点进行循环遍历的几点：
/// 1. 💥报错，while let进行了赋值，borrow的生命周期延长到循环结束，再对visitor赋值引起冲突
/// while let Some(next) = visitor.borrow().right.as_ref().map(Rc::clone) {
///     visitor = next;
/// }
///
/// 2. 克隆一份visitor，用克隆出来的borrow，不影响visitor赋值。clone * 1, borrow * 1
/// while let Some(next) = visitor.clone().borrow().right.as_ref().map(Rc::clone) {
///     visitor = next;
/// }
///
/// 3. 把上面2的逻辑拆进循环内，循环上多borrow了一次.clone * 1, borrow * 2
/// while visitor.borrow().right.is_some() {
///     let tmp = Rc::clone(&visitor);
///     visitor = tmp.borrow().right.as_ref().map(Rc::clone).unwrap();
/// }
///
/// 4. 无需克隆visitor，borrow都在当前语句结束时销毁，不影响visitor赋值。循环上多borrow了一次. clone * 0, borrow * 2
/// while visitor.borrow().right.is_some() {
///     let next = visitor.borrow().right.as_ref().map(Rc::clone).unwrap();
///     visitor = next;
/// }
///
/// 5. 🟢直接loop，条件判断上减少一次borrow，也无需克隆visitor. clone * 0, borrow * 1
/// loop {
///     let next = visitor.borrow().right.as_ref().map(Rc::clone); // 🔐 上锁、克隆
///     if let Some(next_right) = next {
///         visitor = next_right; // 🔓 此时上一行的 next 生命周期已死，锁安全释放！完美赋值！
///     } else {
///         break;
///     }
/// }
struct Solution4;
impl Solution4 {
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        if root.is_none() {
            return;
        }
        let mut cur = root.as_ref().map(Rc::clone);
        while let Some(node_rc) = cur {
            let left = node_rc.borrow_mut().left.take();
            if left.is_some() {
                let mut visitor = left.as_ref().map(Rc::clone).unwrap();
                while visitor.borrow().right.is_some() {
                    let next = visitor.borrow().right.as_ref().map(Rc::clone).unwrap();
                    visitor = next;
                }

                let old_right = node_rc.borrow_mut().right.take();
                visitor.borrow_mut().right = old_right;
                node_rc.borrow_mut().right = left;
            }
            cur = node_rc.borrow().right.as_ref().map(Rc::clone);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree;

    #[test]
    fn test1() {}
}
