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

    // interior mutability example

    let f = 5;
    //let g = &mut f; // trying to borrow mutably an immutable value
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

// interior mutability

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger, value: 0, max
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("You are over the quote");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("You've used up over 90& of your quota")
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("You've used up over 75% of your quota")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_message: RefCell<Vec<String>>
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_message: vec![]
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            self.sent_message.borrow_mut().push(String::from(msg));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_message.borrow_mut().len(), 1);
    }
}

// Keeping Track of Borrows at Runtime with RefCell<T>

/*
 * RefCell<T>
 * - borrow -> Ref<T>
 * - borrow_mut -> RefMut<T>
 */

