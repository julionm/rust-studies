use std::ops::Deref;
use std::mem::drop;
use std::rc::Rc;

use crate::List::{Cons, Nil};

fn main() {
    // * in fact, the box is stored on the stack
    // * and the data itself on the heap
    let b = Box::new(5);
    println!("b = {}", b);

    let x = 5;
    let y = MyBox::new(x);

    println!("{}", x);

    // the compilers undertands it as *(y.deref())
    println!("{}", *y);


    let n = MyBox::new(String::from("Rust"));
    hello(&n);

    let c = CustomSmartPointer {
        data: String::from("letter c")
    };

    let d = CustomSmartPointer {
        data: String::from("letter d")
    };

    println!("Drop early");
    drop(c);

    println!("Finished main");

    let a1 = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("the reference count is: {}", Rc::strong_count(&a1));
    let b1 = List::Cons(4, Rc::clone(&a1));
    println!("the reference count is: {}", Rc::strong_count(&a1));
    {
        let b2 = List::Cons(3, Rc::clone(&a1));
        println!("the reference count is: {}", Rc::strong_count(&a1));
    }

    println!("the final reference count is: {}", Rc::strong_count(&a1));
}

// * cons list from functional programming
// ! infinite size list
pub enum List {
    Cons(i32, Rc<List>),
    Nil,
}
// ? List needs a space in memory equal to i32 + List.len(), to know
// ? which is the size of List, it calculates the Const inside the List 
// ? again and again, doin it infinitely

// ? tuple struct type
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0 // valor inicial da tupla
    }
}

fn hello(name: &str) {
    println!("Hello: {}", name);
}

struct CustomSmartPointer {
    data: String
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping it with data: {}", self.data);
    }
}
