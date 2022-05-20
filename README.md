# Rust Knowledge

> Current Book Chapter: 7.3

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
```
struct User {
    username: String,
    password: String,
    age: u16,
    enable: bool
}
```
> OBS: additionaly is possible to create mutable struct instances using:
```
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

---

## Future Concepts
- Lifetimes
- Iterators
- Deref coertions
- Automatic Referencing and Deferencing
- Generic type parameter
- Panics
- Cargo Workspaces
- Unsafe Rust (https://doc.rust-lang.org/nomicon/)

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
    ```

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
    ```

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
- The module's code are stored inside files with it's names, therefore, we use the:
  > ``` pub mod new_module; ```  
  to call it inside the file we need to use
  
