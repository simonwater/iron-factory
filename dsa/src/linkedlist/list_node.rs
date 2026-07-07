#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    pub fn from_vec(vals: Vec<i32>) -> Option<Box<Self>> {
        let mut next: Option<Box<ListNode>> = None;
        for &val in vals.iter().rev() {
            let mut node = ListNode::new(val);
            node.next = next;
            next = Some(Box::new(node));
        }
        next
    }

    pub fn from_vec1(vals: Vec<i32>) -> Option<Box<Self>> {
        let mut dumy = ListNode::new(0);
        let mut cur = &mut dumy;
        for val in vals {
            let node = ListNode::new(val);
            cur.next = Some(Box::new(node));
            cur = cur.next.as_mut().unwrap();
        }
        dumy.next
    }

    pub fn print(&mut self) {
        let mut cur = Some(self);
        while let Some(node) = cur {
            print!("{} ", node.val);
            cur = node.next.as_deref_mut();
        }
        println!();
    }
}

pub fn print_list(mut list: Option<Box<ListNode>>) {
    while let Some(node) = list {
        println!("{}", node.val);
        list = node.next;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let list = ListNode::from_vec(vec![1, 2, 3, 4, 5]);
        print_list(list);

        let list = ListNode::from_vec1(vec![11, 22, 33, 44, 55]);
        print_list(list);
    }
}
