/// [449. 序列化和反序列化二叉搜索树](https://leetcode.cn/problems/serialize-and-deserialize-bst/)
///
use crate::binary_tree::TreeNode;
use std::cell::{Ref, RefCell};
use std::rc::Rc;
struct Codec {}

impl Codec {
    fn new() -> Self {
        Self {}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut ans = Vec::with_capacity(32);
        self.dfs(&root, &mut ans);
        ans.iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }

    fn dfs(&self, root: &Option<Rc<RefCell<TreeNode>>>, ans: &mut Vec<i32>) {
        if let Some(root_rc) = root {
            let node = root_rc.borrow();
            ans.push(node.val);
            self.dfs(&node.left, ans);
            self.dfs(&node.right, ans);
        }
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        if data.is_empty() {
            return None;
        }
        let nums = data
            .split(",")
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let mut pos = 0usize;
        self.make_node(&nums, &mut pos, i32::MIN, i32::MAX)
    }

    fn make_node(
        &self,
        nums: &[i32],
        pos: &mut usize,
        min: i32,
        max: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if *pos == nums.len() {
            return None;
        }
        let num = nums[*pos];
        if num <= min || num >= max {
            return None;
        }
        let mut node = TreeNode::new(num);
        *pos += 1;
        node.left = self.make_node(nums, pos, min, num);
        node.right = self.make_node(nums, pos, num, max);
        Some(Rc::new(RefCell::new(node)))
    }
}
