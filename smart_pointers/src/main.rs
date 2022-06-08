use std::ops::Deref;

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
}

// * cons list from functional programming
// ! infinite size list
enum List {
    Cons(i32, Box<List>),
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
