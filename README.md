# Joined The #100DaysOfCode Challenge
I commit myself to:
- Update this repository everyday with some new content
- Study at least 1 hour of Rust (or Haskell) everyday
- Think a way to automatize some of the updates using Rust

# Rust Knowledge

> Current Book Chapter: 15

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

---

## Future Concepts
- Lifetimes - **OK**
- Iterators - **OK**
- Deref coertions
- Automatic Referencing and Deferencing
- Generic type parameter - **OK**
- Panics - **OK**
- Cargo Workspaces - **OK**
- Unsafe Rust (https://doc.rust-lang.org/nomicon/)
- Glob
- Variadic Functions

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

## Build and add to PATH

When i finished the program of Chapter 12, i had some doubts about how to use my program through the OS, some amazing guys helped me and that's what they told me:
- `cargo build --release` the released artifact will be on target/debug/{project_name}
- then, in the root directory of the application do:  `cargo install --path .`

## trying to process things

what i could understand from `Box`'s is:
- it haves some defined size, but in the book it doesn't tells how much is this size
- seems to me that some problem with reallocation with vectors is caused, because vec! uses a `Box` under the hoods
- and `Box` is a pointer that is *pushed onto the stack* while the data it points to is allocated in the *Heap*
- 

General questions:
- Pointers are stored inside the Stack because they have fixed size? I remember some part of the book saying it was in the Heap