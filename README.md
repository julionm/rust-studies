# Joined The #100DaysOfCode Challenge
I commit myself to:
- Update this repository everyday with some new content
- Study at least 1 hour of Rust (or Haskell) everyday
- Think a way to automatize some of the updates using Rust

> CURRENT DAYS OF CODE: 18

# Rust Knowledge

> Current Book Chapter: 19.3

Links to the following books i'll read:

- *The Rust Programming Language V2* - https://doc.rust-lang.org/book/title-page.html (I'm here)
- *The Rust Programming Language V1* - https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/README.html
- *The Reference* - https://doc.rust-lang.org/reference/attributes.html
- *The Rustonomicon* - https://doc.rust-lang.org/nomicon/index.html

Some questions I asked myself, to keep track of what I was learning through the reading of ‘’The Rust Programming Language", available at: https://doc.rust-lang.org/book/.

### How does occur the memory allocation?

The called operation “allocation”, occurs when the memory allocator searches in the Heap storage for a space
to allocate some data that don’t have fixed size, so it’s stored in the heap memory the information(pointers, length and capacity)
of the data and in the stack the real data since it only accepts data with fixed size as characters, integers, floating points and tuples.

### How does occur the  pushing onto the stack operation?

The stack memory follows the rules of LIFO (last in, first out), organizing all data that has fixed size in a sort of stack, so the most recently add data is always the ones to be erased from memory.

### What is Ownership?

All variables in Rust have a value that it’s called his owner, it’s only possible to have one owner at a time and if the owner goes out of scope the value will be dropped.

That is an strategy Rust uses to manage not-used variables saving the ownership property inside every variable that has been created, each variable can have one owner at a time, and if the owner scope ends the value of the variable is dropped, so Rust uses it to manage garbage memory, dropping memory that has no more use since the scope it was in end.

### What is the difference between pointers and reference?

Pointer always point to some stack stored values and can point to null values, reference ALWAYS points to a valid value. When the pointer is moved to another variable, the previous has no more value and can’t be used anymore, reference still is maintained in the previous var since the reference itself is passed to the new var.

### What is Borrowing?

Borrowing is the technique in rust of literally borrowing reference of a variable to another, when you pass a reference to some data to a function, the value is borrowed to the variable in the scope of the function to maintain the original value untouched even if the function is over and the scope ended, preserving this way the old state of the variable passed as parameter.


### What is the purpose of structs and for what it's used for?

Structs are used to store organized data of the same subject, so if we need to save some information about a User, we can create a struct to store in it the username, password, age and if it's enable inside one User instance. Creating an variable of the type created in the struct results in an instance of that struct saved in memory. Their structure follows something like this:
```rust
struct User {
    username: String,
    password: String,
    age: u16,
    enable: bool
}
```
> OBS: additionaly is possible to create mutable struct instances using:
```rust
let mut user = User {
    username: String::from("julio_negri"),
    password: String::from("strong_pwd"),
    age: 21,
    enable: true
}
```

### Name the differences between functions and methods.

Both execute code inside them, may receive parameters and return values, but instead of a function mehtos are defined within the context of a struct, enabling it to access the struct properties through the self parameter.

### What is an associated function?

The associated functions are normally used to generate new instances of an struct, like constructors in other languages, they don't receive the self attribute, used by the methods to acces the inner properties of an struct and are detached of the context of it.

### How does works the privacy boundary in Rust?

We start saying that all functions, structs, methods, enums and constants are treated as they are private by default. What the  
 privacy boundaries actually does is secure a code that should never be used outside it's context, making your code safier  
 because it won't receive changes that could break some of their defined rules and behaviours.

### What's the problem when saving a reference to an Vector index, changing the vector and accessing the reference later?

The answer to the question is explaining how works the vectors in the background.  
Rust for its own, store all vector's values next to each other in memory, considering the reference to index 0 is stored inside an X variable, if new values are added to the vector, there's a chance it's not enough memory available allocated in that space, if that's the case then the memory allocator will copy all the values and transfer to a new space with enough size. Therefore, our declared X variable will reference a deallocated memory space, making it invalid, the borrowing rules prevent this to happen.  
Because of this situation, there's a rule that states you can't have mutable and immutable references in the same scope.

### How does the Rust unwinding happends?

When some Rust program suffers a panic error, the default behavior is to unwind through the functions it entered and cleans up all the stack memory, the process itself is a lot of work to the compiler, so Rust gives the option to just abort the program and leave the clean process to the OS.

### How to use the `?` operator for error handling?

The `?` operator is a replace for `match` expressions to catch errors. If a method returns a `Result` enum, it'll automatically check if occured an exception and propagate it, returning the error immediately when it's catched. The only counterpart is that by default, it converts the error object to the one declared in the function declaration, so it's better used in cases where only one kind of error is possible.

### What is _Monomorphization_?

When using Generics in any language there's a concern in the performance of the code, but in rust with the _monomorphization_ it's not a problem at all, the process create in compile time all the code that will be used. If there´s a enum over a generic type and it's implemented using a `String` and a `u32` values, two enums are created for each use, ensuring a performatic and readable code.

### What's the difference between `Copy` and `Clone` ?

`Copy` is implicit, inexpensive and cannot be re-implemented 
`Clone` is explicit, may be expensive and may be re-implemented

The `Clone`  is explicit, therefore it needs to be declared, as it's a more geenric trait, it can be reimplemented in other types to do arbitrary stuff and since it´s more generic than the other one the values that implement `Copy` may implement `Clone` as well.

`Copy` is a bitwise copy, therefore it copies every bit of the value to another memory location (variable) using the `memcpy()` function.

### What is a _Reference Cycle_ and how to prevent it from leaving a *Memory Leak*?

A reference cycle happens when you have different references pointing to each other, it may happen in many cases but when you think of a linked list where the values are linked using references to other list items, for example, it may occur a cycle problem when you point list item a to list item b and vice-versa, if you're using some `Rc<T>` for handling the multiple times the list item could be referenced, you may use the `Weak<T>` smart pointer to help you with that.

When using the `Rc::clone` you increase the `strong_count` inside it, but the `clone` method shares the ownership so, in the end of the program, it'll not be cleaned up. But when you create a `Weak<T>` reference, instead, it increses the `weak_count` counter, this as a weak reference to the value, will be freed in the end of the execution.

In addition, the methods for manipulating an `Rc<T>` to use the `Weak<T>` smart pointer are:
- `Rc::downgrade` receives a `Rc` reference and returns a new `Weak<T>` smart point to be used
- `Weak::upgrade` this'll return a new `Option<Rc<T>>` that, if it's still available, may be mapped to an `Rc<T>` again.

---

## Future Concepts

To fix my knowledge, in times to times, i'll try to review these items and defining them with my own words to keep remembering everything.

- Lifetimes - **OK**
- Iterators - **OK**
- Deref coertions - **OK**
- Automatic Referencing and Deferencing - **OK**
- Generic type parameter - **OK**
- Panics - **OK**
- Cargo Workspaces - **OK**
- Unsafe Rust - **OK**
- Glob
- Variadic Functions
- Associated Types - **OK**
- Derive
- Halting Problem
- Hamming Algorithm
- Review Binary Tree Algorithm
- FFI (Foreign Function Interface) - **OK**
- Bookkeeping
- Mutex/Arc/Cell - **OK**
- Cool things that museun told about that i want to learn
  - cache locality
  - spatial locality
  - cache coherence (for leveled cache and tlb)
  - cache line
  - cache miss
  - translation lookaside buffer (tlb) 


---

## General Annotations
- Rust handles only one mutable reference per data type at a time
- Data race, steps to happen: 
    - Two ore more pointers accessing same data at the same time
    - One of the pointers is being used to write data
    - There’s no mechanism being used to synchronize access to the data
- String slices, variables that reference the middle of the string, rather than the first element
- Ranges that start in zero can abstract it, making 0..5 = ..5
- It's possible to add other struct values into another using the syntax
    ```rust

    fn main() {
        // user 1 definition

        let user2 = User {
            username: String::from("julio_negri"),
            ..user1
        }
    }

    ```
    If the user1 had only brought, stack-only data, than the user1 instance would still be valid
    through the rest of the scope, but in this the object has String's inside, so the values are
    moved to user2 and user1 is invalidated in the scope.
- Exist two more ways to create structs
    ```rust

    struct Coord(i32, i32);

    struct Color_RGB(str, str, str);

    fn main() {

        let coord_init = Coord(10, 10);

        let color_bg = Color_RGB("ff", "ff", "ff");

    }

    ```
- Enums may have his own methods
- When using the `match`, the `_` is used to represent the default value
- Packages are the projects we create using cargo
- Modules are used to separate source code, they control the privacy of the items
- Crates are the dependencies and libs we add to the project
- The module structure of the crate is named *module tree*
- The navigation through modules are like navigating through a file system, having two ways to achieve  
  the wanted function
- The child modules can see ancestor modules because it can see the context where they're defined, otherwise  
  ancestor modules can't see their child's implementations
- Structs and enums are treated different when talking about privacy, if the struct is public  
  all of it's fields are private by default, so if a field should be public it has to have the `pub` keyword
- Enums variants though, follow the struct privacy, if it's public then all variants are public too
- When importing a function inside a module, the right way to do it is importing the all the parent module to get the function  
  you need, because it's the more idiomatic way, making it more readable and padronized
- If the import is an enum, struct or any other element it should be used the full path to that element
- Its possible to rename conflicting named imports by using the `as` keyword
- The module's code are stored inside files with it's names, therefore, we use the: to call it inside the file we need to use:
  > ``` pub mod new_module; ```
- rust vectors can store any kind of value, it handle the generic type like `Vec<T>`, when all values have the same type, Rust  
  infers the type to the array making `vec![1,2,3] = Vec<i32>`.
- just like structs, when the scope ends vectors and their values are dropped (freed from memory)
- **OBS:** Rust must know the type of the values inside a vector to know how much memory will be needed in heap to store  
  those values
- `push_str` doesn't take ownership of the value passed to it
- Rust doesn`t support string indexing
- Strins are stored as byte vectors using `Vec<u8>`, usually each value corresponds for a unique letter, but there's linguistics cases where it don't happen as expected, so to prevent this error cases, rust gives an error when we try to index a string value
- there`s another reason rust doens't permit string indexing: performance, as a byte vector Rust can't guarantee the expected time, cause to reach the wanted index Rust would need to run through all of indexes 'till find the one the user requested
- by default, rust uses a SplitHash algorithm in HashMap, it's much slower than others but it's more secure too so the trade-off, less performance, more security is good
- unrecoverable errors are those that make the application unable to continue running or that may cause a failure of security, like the buffer overread problem, that can cause some invaders to access other memory locations from the reference apointing to a null space
- Cases to `panic!`: to panic the code is a good choice when it we'll be in a _bad state_, i.e. when other code passes wrong values to your lib, this would be a good choice to panic, to alert the user that he's passing the wrong values during development, causing it not breaking when on production
- the _bad state_ happens when your code relies in the existence of a username value, for example, you won't check every time you use the `username` variable if it has some content, cause your code really need this, so for better functioning it should warn the user that it MUST be defined
- **out-of-bound memory access**: access a memory that is no longer managed by the program
- **function contracts**: means that the correct behaviour of a function relies on it's parameters **meeting** the requirements this function needs
- when using angle brackets syntax to make a function or struct generic: `struct Point<T> || fn func<T> (var: T)` the correct is to say that the function or struct is generic over some type `T`
- traits are used to describe shared behaviour in rust structs
- the dangling reference is a problem where a reference points to other value than the value it was suppose to
- `cargo clean` IT SAVES LIVES
- in rust iterators are *lazy*, that is they have no effect untill you call methods to use it
- *iterators* are one of Rust's *zero-cost abstractions*, because in the end they're all compiled to the same binary code a `for` loop will be, so they give a more readable code by not losing performance, it's a gain-gain situation
- `///` triple slash comments are used for documenting in rust, they accept even markdown annotations in it
- `//!` this kind of comments are used for whole descriptions, used in *crate root files* or in modules declaration
- the types where you can't know their size during compile time is a *recursive type*
- the `Box` is stored in the *Stack* rather it's data that it's stored in the *Heap*
- *deref coercion* is a conveniece Rust compiler do on function's or methods arguments, it happens when the reference type doesn't match with the ones expected for the function, then a continuing call to the deref methods happen 'till the types match
- the `Drop` **trait** is used to do some action before the variable goes out of scope, so types that implement this trait may do something before those variables go out of scope
- `Rc<T>` is for *reference counting*, it's used for counting how many references point to the same object in memory to do a safe clean up, it enables for a value to have multiple owners or the so called multiple ownership, if there are no references anymore the value can be cleaned up without invalidating any reference
- the *reference counting* is needed when different parts of the program have to use the same value, but it's impossible to know which one will finish the use in compile time
- `Rc::clone(&a)` is much more performative than `a.clone()`, cause it only increses the reference count rather than creating a new `a` in the memory
- `Drop` trait automatically decreases the `Rc<T>` when the reference goes out of scope
- `RefCell<T>` basically wraps some `unsafe` Rust code, to change a value that has been referenced before
- *Fearless Concurrency* is how Rust calls it's features to treat concurrency issues using ownership concepts, with these new ideas, they could bring the massive amount of concurrency bugs to be compile time bugs rather than runtime, preventing unexpected errors and crashes
- the *shared memory concurrency* is like the multiple ownership covered by smart pointers, where may exist multiple owners of the same data in memory, in Rust this concepts fix perfectly to handle problems of shared memory
- `Mutex` *mutual exclusion*
- `mpsc` *multiple producer, single consumer*
- the `Mutex::lock` makes the current execution thread to wait untill the value is available, if another thread is using before
- the `MutexGuard` smart pointer unlocks the data it's pointing when it goes out of scope
- `Arc<T>` *atomic reference counting*
- `Mutex<T>` just like the `Cell` family, provides *interior mutability*
- the `Send` trait enables a type to have his ownership shared between threads
- `Rc<T>` can't implement `Send` cause it's internal counting could be changed at the same time
- the `Sync` trait can be implemented by types `T` where `&T` may be used between multiple threads at the same time, likely the `Mutex<T>` implements this trait
- trait bounds works as generics, but with a differencial, the trait objects allow it to be filled with any type that implements the trait at runtime, as the follow code:  
```rust

pub struct Screen<T: Draw> {
  pub components: Vec<T>
}

impl<T> Screen<T>
where
  T: Draw
  {
    pub fn run(&self) {
      for component in self.components.iter() {
        component.draw();
      }
    }
  }
  // this code will only accept one concrete type at a time
  // causing to be impossible

```
- trait objects do the rust compiler generate code to treat *dynamic dispatch* as it's impossible to know which will be the types received, instead of generics that use *static dispatch*
- the best introduction for Stack and Heap: **https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/the-stack-and-the-heap.html#moving**
- *Stack* is not shared through threads, *Heap* is
- Rust uses a memory management of type *RAII*
- Places where patterns can be used:
  - `match` arms (must be exhaustive, so need to embrace all possibilities)
  - `if let` expressions: the compiler doesn't check exhaustiveness, hence, it's not checked all possibilities, differently of `match` expressions
  - `while let` conditional loops:
  ```rust

  let mut stack = vec![1,2,3];
  while let Some(top) = stack.pop {
    // some awesome code
  }

  ```
  - `for` loops: in this type of loop, the pattern is the first value after `for` statement
  - `let` statements
  - function parameters
- *refutable patterns*: patterns where it matches all possibilities `let x = 5` is an example
- *irrefutable patterns*: patterns where only one or few are matched, an example for this is:
  ```rust
    if let Some(x) = value {

    }
  ```
  if the `value` is None, it wouldn't match
- *associated types*, basically work as generic type parameters, but they don't allow multiple implementations of a trait, when using generic parameters, it's possible to create multiple implementations with different types, associated types give a possibility to use generics without allowing multiple implementations of the same trait for a struct

---

## ANNOTATIONS ABOUT LIFETIMES

I prefered to create a separate part for lifetimes due to the amount of content and complexity it took me to learn it.

- lifetimes is the time where the references keep alive
- the lifetime annotation use in function signatures, creates a contract in the function, that ensures the lifetime of the returned value is the same as the arguments received in the function, as the following example:
  ```rust

  fn longest<'a>(arg1: &'a str, arg2: &'a str) -> &'a str {

  }
  ```
- _lifetime elisions_ are a set of rules that rust created to assist programmers that used to write duplicate code a lot in the past, when declaring lifetimes many times it's needed to declare the lifetime multiple times, in the function, in the argument and in the return type so the rust team made the borrow checker do it in compile time, so in some cases the lifetimes are infered in the code
- those inferences are not full, if rust doesn't have full information to infer, it won't guess the lifetime of the parameter or the return type
- lifetimes on functions and methods are called *input lifetimes*
- lifetimes on return values are *output lifetimes*
- there's three rules to the lifetime inferation happen, if it fails to figure out the lifetime for any reference an error will be returned, these rules apply for ```fn``` definitions and ```impl``` blocks
  - FIRST RULE: Rust gives a lifetime for each parameter that's a reference in the function, fn with 1 argument has 1 lifetime parameter
  - SECOND RULE: if there's exatcly one input *lifetime parameter*, the same lifetime is used on the output lifetime parameter
  - THIRD RULE: on methods, the self lifetime is assigned to all output lifetime parameters
- the static lifetime: this lifetime indication, tell rust those references will live through the entire execution of the program, is used as ```'static```, the lifetime for all string literals is static
- example usage of generics + lifetimes + trait bounds:
  ```rust

  fn longes_with_an_announcemennt<'a, T>(x: &'a str, y: &'a str, ann: T) {
    ...
  }

  ```

---

## Build and add to PATH

When i finished the program of Chapter 12, i had some doubts about how to use my program through the OS, some amazing guys helped me and that's what they told me:
- `cargo build --release` the released artifact will be on target/debug/{project_name}
- then, in the root directory of the application do:  `cargo install --path .`

---

### Smart Pointers

- reference is a kind of pointer, but it doesn't take ownership of the value it points to, just borrow them
- `String`s use pointers under the hood
- they're data structures that act like a pointer, but save some metadata and capabilities like: `String`s and `Vec<T>`
- normally they are implemented using structs, but instead of normal structs they implement the `Deref` and `Drop` traits

---

### Box

- what i could understand from `Box`'s is:
  - `Box` is a *Smart Pointer*
  - it stores values(data) on the *Heap*
  - the pointer or the *memory address* is pushed onto the *Stack*
  - as the box stores an *memory address* it has a fixed size, so in cases like in the example with recursively types that the compiler infer an infinite size  
    rather than using the value we intend, we can wrap it inside a `Box` that HAS a fixed size.
  - because `Box<T>` implements the `Deref` trait, it allows the `Box` values to be treated as references
  - with the `Drop` trait we can guarantee that in the end of the scope where `Box` was declared, either the value stored in *Heap* and the pointer stored  
    in *Stack* are both cleaned

General questions:
- Where are pointers stored? R: In the stack, as they're memory addresses basically, they have fixed size

---

## different concepts i learned through this journey

- **adhoc specialization**: specialization is a way to treat things differently inside a function based on the args passed  
  for example, in `vec!` are different treatments based on the initial values passed to the macro, empty, primitive types and `String`.
  Adhoc in this context is a *user made* specialization, cause this specialization is only located in unstable Rust, so the
  *adhoc specialization* are basically a group of come specialization created by a user to implement the same behavior
- **destructor**: common concept of functions that cleans instances
- **deadlock**: two threads waiting for each other execution to continue, creating a no exit situation, in other words creating a *deadlock*
- **duck typing**: if it walks like a duck and quacks like a duck, it must be a duck!
- **static dispatch**: the compiler knows which function with which types you're calling in compile time
- **dynamic dispatch**: with dynamic, the compiler will only know which calls you're doing in runtime, so the code generated when using *static* and when using *dynamic* are different, when *dynamic* the program will have to discover in runtime which methods are called
- **RAII**: the type of memory management Rust uses, it's called *Resource Acquisition is Initialization*, when occurs an initialization the variable owns the resource, so when it goes out of scope, the variable is dropped and the memory freed
- **Flip Flop**: edge-triggered latch, it's how modern computers store binary data, changing the inner data deppending on the inputs the system receives
- **Latch**: eletronic circuit... **STUDY MORE**
- **ABI**: the *application binary interface* defines how to call the function at the assembly level, using the "C" ables to use functions of the C programming language
- **Mangling**: is when the compiler changes the function name to add more information so other parts of compilation process can use it
- **Operator Overloading**: it's the way to overload basic operations of operators like `+` depending on the values passed

---

### borrowing rules

- in any given time, you can have *either* (but not both of) one mutable and many immutable references
- all references must be valid

---

### mutex rules

- try to acquire the lock before using the data
- when you finish to work with the data, unlock the data for other threads be able to get it
 
--- 

### unsafe superpowers

- Dereference raw pointers - using raw pointers is cool, but needs to be careful
- Call an unsafe function or method - cool too, but not soo much use to this
- Access or modify a mutable static variable - maybe nice, but maybe spend more minutes can prevent this usage
- Implement an unsafe trait - cool but i need to study it's usage better to know the possibilities
- Access fields of `union` s - only needed when integrating with C code


### unsafe superpowers annotations

- **raw pointers**: they work just as any normal pointer in C/C++, references in rust are basically pointers with memory safety, can be immutable or mutable, immutable means that the pointer can't be directly assigned to after being dereferenced.
  - are allowed to have mutable and immutable references at the same time
  - are allowed to be null
  - dont implement any automatic cleanup
  - aren't guaranteed to point to valid memory

---

### Useful Links

- Memory management - https://www.youtube.com/user/Computerphile/search
- Memory management in Rust - https://deepu.tech/memory-management-in-rust/
- Arcticle about Dynamic Storage Allocation - https://users.cs.northwestern.edu/~pdinda/ics-s05/doc/dsa.pdf