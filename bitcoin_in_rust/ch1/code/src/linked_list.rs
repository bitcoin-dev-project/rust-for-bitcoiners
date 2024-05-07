#![allow(unused)]

struct LinkedList<T> {
    head: Node<T>,
}

struct Node<T> {
    val: T,
    next: Box<Node<T>>, // trailing commas are allowed in rust
}

/*
Rust compiler tries to fit all the values in the process stack.
Why? Because Rust was created to produce efficient code, which is fast and uses
less memory. And accessing memory in the process stack is much faster and has 0 allocation overhead
than accessing memory in the heap which requires one indirection and one allocation through a sys call.
*/

// integer types of rust are i8, i16, i32, ..., i128
