use std::{cell::RefCell, rc::Rc};

struct Node {
    pub start_time: i32,
    pub end_time: i32,
    pub left: Option<Rc<RefCell<Node>>>,
    pub right: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(start_time: i32, end_time: i32) -> Self {
        Self {
            start_time,
            end_time,
            left: None,
            right: None,
        }
    }
}

// 递归
struct MyCalendar {
    node: Option<Rc<RefCell<Node>>>,
}

impl MyCalendar {
    fn new() -> Self {
        MyCalendar { node: None }
    }

    fn book(&mut self, start_time: i32, end_time: i32) -> bool {
        if let Some(root) = self.node.as_ref() {
            Self::dfs(Rc::clone(root), start_time, end_time)
        } else {
            self.node = Some(Rc::new(RefCell::new(Node::new(start_time, end_time))));
            true
        }
    }

    fn dfs(node_rc: Rc<RefCell<Node>>, start_time: i32, end_time: i32) -> bool {
        let (cur_start, cur_end, left, right) = {
            let node = node_rc.borrow();
            (
                node.start_time,
                node.end_time,
                node.left.as_ref().map(Rc::clone),
                node.right.as_ref().map(Rc::clone),
            )
        };
        if start_time >= cur_end {
            // to right
            if let Some(right_rc) = right {
                return Self::dfs(right_rc, start_time, end_time);
            } else {
                node_rc.borrow_mut().right =
                    Some(Rc::new(RefCell::new(Node::new(start_time, end_time))));
                return true;
            }
        } else if end_time <= cur_start {
            // to left
            if let Some(left_rc) = left {
                return Self::dfs(left_rc, start_time, end_time);
            } else {
                node_rc.borrow_mut().left =
                    Some(Rc::new(RefCell::new(Node::new(start_time, end_time))));
                return true;
            }
        } else {
            return false;
        }
    }
}

// 迭代
struct MyCalendar2 {
    node: Option<Rc<RefCell<Node>>>,
}

impl MyCalendar2 {
    fn new() -> Self {
        MyCalendar2 { node: None }
    }

    fn book(&mut self, start_time: i32, end_time: i32) -> bool {
        if let Some(root) = self.node.as_ref() {
            let mut visitor = Rc::clone(root);
            loop {
                let (cur_start, cur_end, left, right) = {
                    let node = visitor.borrow();
                    (
                        node.start_time,
                        node.end_time,
                        node.left.clone(),
                        node.right.clone(),
                    )
                };

                if start_time >= cur_end {
                    if let Some(right_rc) = right {
                        visitor = right_rc;
                    } else {
                        visitor.borrow_mut().right =
                            Some(Rc::new(RefCell::new(Node::new(start_time, end_time))));
                        return true;
                    }
                } else if end_time <= cur_start {
                    if let Some(left_rc) = left {
                        visitor = left_rc;
                    } else {
                        visitor.borrow_mut().left =
                            Some(Rc::new(RefCell::new(Node::new(start_time, end_time))));
                        return true;
                    }
                } else {
                    return false;
                }
            }
        } else {
            self.node = Some(Rc::new(RefCell::new(Node::new(start_time, end_time))));
            true
        }
    }
}
