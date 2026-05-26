use crate::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

/// [105. 从前序与中序遍历序列构造二叉树](https://leetcode.cn/problems/construct-binary-tree-from-preorder-and-inorder-traversal/)
struct Solution;
use std::collections::HashMap;
impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let n = preorder.len();
        let mut map = HashMap::new();
        for (idx, &val) in inorder.iter().enumerate() {
            map.insert(val, idx);
        }
        Self::build_helper(&preorder, &inorder, &map, 0, n - 1, 0, n - 1)
    }

    pub fn build_helper(
        preorder: &[i32],
        inorder: &[i32],
        map: &HashMap<i32, usize>,
        pre_start: usize,
        pre_end: usize,
        in_start: usize,
        in_end: usize,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if pre_start > pre_end || in_start > in_end {
            return None;
        }

        let val = preorder[pre_start];
        let mut node = TreeNode::new(val);
        let k = map.get(&val).copied().unwrap();
        let left_len = k - in_start;
        node.left = if k == 0 {
            None
        } else {
            Self::build_helper(
                preorder,
                inorder,
                map,
                pre_start + 1,
                pre_start + left_len,
                in_start,
                k - 1,
            )
        };
        node.right = Self::build_helper(
            preorder,
            inorder,
            map,
            pre_start + left_len + 1,
            pre_end,
            k + 1,
            in_end,
        );

        return Some(Rc::new(RefCell::new(node)));
    }
}

/// [106. 从中序与后序遍历序列构造二叉树](https://leetcode.cn/problems/construct-binary-tree-from-inorder-and-postorder-traversal/)
struct Solution2;
impl Solution2 {
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let n = inorder.len();
        let mut map = HashMap::new();
        for (idx, &val) in inorder.iter().enumerate() {
            map.insert(val, idx);
        }
        Self::build_helper(&inorder, 0, n - 1, &postorder, 0, n - 1, &map)
    }

    pub fn build_helper(
        inorder: &[i32],
        in_start: usize,
        in_end: usize,
        postorder: &[i32],
        post_start: usize,
        post_end: usize,
        map: &HashMap<i32, usize>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if in_start > in_end || post_start > post_end {
            return None;
        }
        let val = postorder[post_end];
        let mut node = TreeNode::new(val);
        let k = map.get(&val).copied().unwrap();
        let left_len = k - in_start;
        node.left = if k == 0 {
            None
        } else {
            Self::build_helper(
                inorder,
                in_start,
                k - 1,
                postorder,
                post_start,
                post_start + left_len - 1,
                map,
            )
        };
        node.right = if post_end == 0 {
            None
        } else {
            Self::build_helper(
                inorder,
                k + 1,
                in_end,
                postorder,
                post_start + left_len,
                post_end - 1,
                map,
            )
        };
        Some(Rc::new(RefCell::new(node)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree;

    #[test]
    fn test1() {
        let preorder = vec![3, 9, 20, 15, 7];
        let inorder = vec![9, 3, 15, 20, 7];
        let ans = tree![3, 9, 20, null, null, 15, 7];
        assert_eq!(Solution::build_tree(preorder, inorder), ans);

        let preorder = vec![1, 2];
        let inorder = vec![2, 1];
        let ans = tree![1, 2];
        assert_eq!(Solution::build_tree(preorder, inorder), ans);

        let preorder = vec![1, 2, 3];
        let inorder = vec![1, 3, 2];
        let ans = tree![1, null, 2, 3];
        assert_eq!(Solution::build_tree(preorder, inorder), ans);
    }

    #[test]
    fn test2() {
        let inorder = vec![9, 3, 15, 20, 7];
        let postorder = vec![9, 15, 7, 20, 3];
        let ans = tree![3, 9, 20, null, null, 15, 7];
        assert_eq!(Solution2::build_tree(inorder, postorder), ans);

        let inorder = vec![-1];
        let postorder = vec![-1];
        let ans = tree![-1];
        assert_eq!(Solution2::build_tree(inorder, postorder), ans);
    }
}
