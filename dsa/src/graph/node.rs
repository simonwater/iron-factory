use std::cell::RefCell;
use std::rc::Rc;

/// 有循环引用的图节点定义
/// 这种定义如果针对短时间运行的脚本、cli等，使用起来会比较简便，进程执行完后操作系统会回收内存
/// 但针对服务端常驻进程则会有严重的内存泄漏问题
#[derive(Eq, PartialEq, Debug)]
pub struct Node {
    val: i32,
    neighbors: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(val: i32) -> Self {
        Node {
            val,
            neighbors: Vec::new(),
        }
    }

    pub fn add_neighbor(&mut self, node: Rc<RefCell<Node>>) {
        self.neighbors.push(node);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn danger_test() {
        // 如果是常驻进程，会发生内存泄漏
        let node_a = Rc::new(RefCell::new(Node::new(1)));
        let node_b = Rc::new(RefCell::new(Node::new(2)));
        node_a.borrow_mut().add_neighbor(Rc::clone(&node_a));
        node_a.borrow_mut().add_neighbor(Rc::clone(&node_b));
        node_b.borrow_mut().add_neighbor(Rc::clone(&node_b));
        node_b.borrow_mut().add_neighbor(Rc::clone(&node_a));

        println!(
            "A 的邻居的值是: {} {}",
            node_a.borrow().neighbors[0].borrow().val,
            node_a.borrow().neighbors[1].borrow().val
        );
        println!(
            "B 的邻居的值是: {} {}",
            node_b.borrow().neighbors[0].borrow().val,
            node_b.borrow().neighbors[1].borrow().val
        );
    }
}
