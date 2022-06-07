fn main() {
    // * in fact, the box is stored on the stack
    // * and the data itself on the heap
    let b = Box::new(5);
    println!("b = {}", b);
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

