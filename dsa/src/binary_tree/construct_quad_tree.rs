use std::cell::RefCell;
/// # [427. 建立四叉树](https://leetcode.cn/problems/construct-quad-tree/)
///
use std::rc::Rc;
pub struct Node {
    pub val: bool,
    pub is_leaf: bool,
    pub left_top: Option<Rc<RefCell<Node>>>,
    pub right_top: Option<Rc<RefCell<Node>>>,
    pub left_bottom: Option<Rc<RefCell<Node>>>,
    pub right_bottom: Option<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(val: bool, is_leaf: bool) -> Self {
        Self {
            val,
            is_leaf,
            left_top: None,
            right_top: None,
            left_bottom: None,
            right_bottom: None,
        }
    }
}
pub struct Solution;

impl Solution {
    //
    pub fn construct(grid: Vec<Vec<i32>>) -> Option<Rc<RefCell<Node>>> {
        if grid.is_empty() {
            return None;
        }
        let n = grid.len();
        Self::construct_node(&grid, 0, n - 1, 0, n - 1)
    }

    fn construct_node(
        grid: &Vec<Vec<i32>>,
        start_r: usize,
        end_r: usize,
        start_c: usize,
        end_c: usize,
    ) -> Option<Rc<RefCell<Node>>> {
        if start_r == end_r && start_c == end_c {
            return Some(Rc::new(RefCell::new(Node::new(
                grid[start_r][start_c] == 1,
                true,
            ))));
        }
        let d = (end_r - start_r) / 2;
        let left_top = Self::construct_node(grid, start_r, start_r + d, start_c, start_c + d);
        let right_top = Self::construct_node(grid, start_r, start_r + d, end_c - d, end_c);
        let left_bottom = Self::construct_node(grid, end_r - d, end_r, start_c, start_c + d);
        let right_bottom = Self::construct_node(grid, end_r - d, end_r, end_c - d, end_c);

        {
            let l_t = left_top.as_ref().unwrap().borrow();
            let r_t = right_top.as_ref().unwrap().borrow();
            let l_b = left_bottom.as_ref().unwrap().borrow();
            let r_b = right_bottom.as_ref().unwrap().borrow();
            if l_t.is_leaf
                && r_t.is_leaf
                && l_b.is_leaf
                && r_b.is_leaf
                && l_t.val == r_t.val
                && l_t.val == l_b.val
                && l_t.val == r_b.val
            {
                return Some(Rc::new(RefCell::new(Node::new(l_t.val, true))));
            }
        }

        Some(Rc::new(RefCell::new(Node {
            val: false,
            is_leaf: false,
            left_top,
            right_top,
            left_bottom,
            right_bottom,
        })))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
