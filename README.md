# Rust Knowledge

Some questions I asked myself, to keep track of what I was learning through the reading of ‘’The Rust Programming Language’, available at: https://doc.rust-lang.org/book/.

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



## Future Concepts
- Dereferencing
- Lifetimes
- Iterators
- Deref coertions
- Unsafe Rust (https://doc.rust-lang.org/nomicon/)

## General Annotations
- Rust handles only one mutable reference per data type at a time
- Data race, steps to happen: 
    - Two ore more pointers accessing same data at the same time
    - One of the pointers is being used to write data
    - There’s no mechanism being used to synchronize access to the data
- String slices, variables that reference the middle of the string, rather than the first element
- Ranges that start in zero can abstract it, making 0..5 = ..5
