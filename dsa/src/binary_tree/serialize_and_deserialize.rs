/// [297. 二叉树的序列化与反序列化](https://leetcode.cn/problems/serialize-and-deserialize-binary-tree/)
use crate::binary_tree::TreeNode;
use std::cell::RefCell;
use std::fmt::Write;
use std::rc::Rc;
pub struct Codec {}

impl Codec {
    pub fn new() -> Self {
        Self {}
    }

    pub fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut ans = String::with_capacity(256);
        Self::serialize_inner(&root, &mut ans);
        ans
    }

    fn serialize_inner(root: &Option<Rc<RefCell<TreeNode>>>, ans: &mut String) {
        if let Some(node_rc) = root {
            let node = node_rc.borrow();
            //相比 ans.push_str(&node.val.to_string()); 可直接把val格式化到ans中，少一次堆分配
            let _ = write!(ans, "{}", node.val);
            ans.push(' ');
            Self::serialize_inner(&node.left, ans);
            ans.push(' ');
            Self::serialize_inner(&node.right, ans);
        } else {
            // None
            ans.push('*');
        }
    }

    pub fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        Self::deserialize_inner(data.as_bytes(), &mut 0)
    }

    fn deserialize_inner(data: &[u8], pos: &mut usize) -> Option<Rc<RefCell<TreeNode>>> {
        while *pos < data.len() && data[*pos] == b' ' {
            *pos += 1;
        }
        match data[*pos] {
            b'*' => {
                *pos += 1;
                None
            }
            _ => {
                let mut flag = 1;
                if data[*pos] == b'-' {
                    flag = -1;
                    *pos += 1;
                }
                let mut num = 0;
                while *pos < data.len() && data[*pos] >= b'0' && data[*pos] <= b'9' {
                    num = num * 10 + (data[*pos] - b'0') as i32;
                    *pos += 1;
                }
                let mut node = TreeNode::new(num * flag);
                node.left = Self::deserialize_inner(data, pos);
                node.right = Self::deserialize_inner(data, pos);
                Some(Rc::new(RefCell::new(node)))
            }
        }
    }
}
