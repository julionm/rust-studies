// #[derive(Debug)]
// enum List {
//     Cons(Rc<RefCell<i32>>, Rc<List>),
//     Nil,
// }

enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
    parent: RefCell<Weak<Node>>
}

use std::cell::RefCell;
use std::rc::{Rc, Weak};
use crate::List::{Cons, Nil};

fn main() {
    
    // let value = Rc::new(RefCell::new(5));

    // let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    // let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));

    // let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    // *value.borrow_mut() += 10;

    // println!("a after = {:?}", a);
    // println!("b after = {:?}", b);
    // println!("c after = {:?}", c);

    // ! REF CYCLE AND MEMORY LEAK

    // let a1 = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    // println!("a initial rc count = {}", Rc::strong_count(&a1));
    // println!("a next item = {:?}", a1.tail());
    // let b1 =  Rc::new(Cons(10, RefCell::new(Rc::clone(&a1))));
    // println!("a rc count after b creation = {}", Rc::strong_count(&a1));
    // println!("b initial rc count = {}", Rc::strong_count(&b1));
    // println!("b next item = {:?}", b1.tail());
    // if let Some(link) = a1.tail() {
    //     *link.borrow_mut() = Rc::clone(&b1);
    // }
    // println!("b rc count after changing a = {}", Rc::strong_count(&b1));
    // println!("a rc count after changing a = {}", Rc::strong_count(&a1));

    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
        parent: RefCell::new(Weak::new())
    });

    let branch = Rc::new(Node {
        value: 5,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
        parent: RefCell::new(Weak::new())
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    
}

// * RefCell allows multiple immutable references
// * Rc allows multiple owners of the same data
// * What be owner of the same data means?
