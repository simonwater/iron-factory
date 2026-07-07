use std::cell::RefCell;
use std::rc::Rc;

pub enum List {
    Cons(RefCell<i32>, Rc<List>),
    Nil,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::smart_pointer::rc_cell::List::{Cons, Nil};

    #[test]
    fn test1() {
        let a = Rc::new(Cons(RefCell::new(1), Rc::new(Nil)));
        let b = a.clone();
        let c = a.clone();
        if let Cons(v, _) = &*c {
            println!("c: {}", v.borrow());
        }
        if let Cons(v, _) = &*a {
            let mut b = v.borrow_mut();
            *b = 11;
            drop(b);
            println!("change by a: {}", v.borrow());
        }

        if let Cons(v, _) = &*b {
            println!("b: {}", v.borrow());
        }
    }

    #[test]
    fn test2() {
        let data = 1;
        let a = Rc::new(RefCell::new(data));
        let b = a.clone();
        let c = a.clone();
        println!("c: {}", c.borrow());

        *b.borrow_mut() *= 10;
        println!("change by b: {}", b.borrow());

        println!("a: {}", a.borrow());
    }
}
