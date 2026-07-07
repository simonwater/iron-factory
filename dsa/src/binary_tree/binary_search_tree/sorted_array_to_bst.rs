/// [108. 将有序数组转换为二叉搜索树](https://leetcode.cn/problems/convert-sorted-array-to-binary-search-tree/)
use crate::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    // 索引定位
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.is_empty() {
            return None;
        }
        Self::make_tree(&nums, 0, nums.len() - 1)
    }

    fn make_tree(nums: &[i32], start: usize, end: usize) -> Option<Rc<RefCell<TreeNode>>> {
        if start == end {
            return Some(Rc::new(RefCell::new(TreeNode::new(nums[start]))));
        }
        if start > end {
            return None;
        }
        let mid = start + ((end - start) >> 1);
        let val = nums[mid];
        let mut node = TreeNode::new(val);
        if mid > 0 {
            node.left = Self::make_tree(nums, start, mid - 1);
        }
        node.right = Self::make_tree(nums, mid + 1, end);
        Some(Rc::new(RefCell::new(node)))
    }
}

pub struct Solution2;

impl Solution2 {
    // 切片
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::make_tree(&nums)
    }

    fn make_tree(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.is_empty() {
            return None;
        }
        let mid = nums.len() / 2;
        let mut node = TreeNode::new(nums[mid]);
        node.left = Self::make_tree(&nums[..mid]);
        node.right = Self::make_tree(&nums[mid + 1..]);
        Some(Rc::new(RefCell::new(node)))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
