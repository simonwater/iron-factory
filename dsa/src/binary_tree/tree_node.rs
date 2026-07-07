use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn from_vec(vals: Vec<Option<i32>>) -> Option<Rc<RefCell<Self>>> {
        if vals.len() == 0 || vals[0].is_none() {
            return None;
        }

        let root = Rc::new(RefCell::new(TreeNode::new(vals[0].unwrap())));
        let mut q = VecDeque::new();
        q.push_back(Rc::clone(&root));
        let mut i = 1;
        while i < vals.len() {
            if let Some(parent) = q.pop_front() {
                if let Some(val) = vals[i] {
                    let left = Rc::new(RefCell::new(TreeNode::new(val)));
                    q.push_back(Rc::clone(&left));
                    parent.borrow_mut().left = Some(left);
                }
                i += 1;
                if i < vals.len() {
                    if let Some(val) = vals[i] {
                        let right = Rc::new(RefCell::new(TreeNode::new(val)));
                        q.push_back(Rc::clone(&right));
                        parent.borrow_mut().right = Some(right);
                    }
                }
                i += 1;
            }
        }
        Some(root)
    }

    pub fn find_node(root: &Option<Rc<RefCell<Self>>>, val: i32) -> Option<Rc<RefCell<Self>>> {
        if let Some(root_node) = root {
            let borrow = root_node.borrow();
            if borrow.val == val {
                return Some(Rc::clone(root_node));
            }
            Self::find_node(&borrow.left, val).or_else(|| Self::find_node(&borrow.right, val))
        } else {
            None
        }
    }
}

// --- 宏定义 ---
#[macro_export]
macro_rules! tree {
    () => { None };
    // 使用 expr 匹配所有的项
    ($($val:expr),*) => {
        {
            // 巧用宏的“字符串化”或直接判断
            let mut v = Vec::<Option<i32>>::new();
            $(
                // 这里利用一个简单的闭包或辅助逻辑来转换 null
                // 假设我们把 null 定义为一个特殊的常量或直接在数组里处理
                v.push(stringify!($val).parse::<i32>().ok());
            )*
            TreeNode::from_vec(v)
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(tree![1, 3, 2, 5], tree![1, 3, 2, 5]);
        assert_eq!(
            tree![2, 1, 3, null, 4, null, 7],
            tree![2, 1, 3, null, 4, null, 7]
        );

        let tree = tree![2, 1, 3, null, 4, null, 7];
        let node = TreeNode::find_node(&tree, 2);
        assert_eq!(node.unwrap().borrow().val, 2);

        let tree = tree![2, 1, 3, null, 4, null, 7];
        let node = TreeNode::find_node(&tree, 4);
        assert_eq!(node.unwrap().borrow().val, 4);

        let tree = tree![2, 1, 3, null, 4, null, 7];
        let node = TreeNode::find_node(&tree, 0);
        assert_eq!(node, None);
    }
}
