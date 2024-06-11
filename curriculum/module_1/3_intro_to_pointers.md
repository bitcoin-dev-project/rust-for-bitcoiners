# Introduction to Pointers

## You might've worked with pointers before

The concept of pointers are unique to languages like C, C++, Rust etc., which
gives explicit control to how the values are laid out in memory.
In object oriented programming languages like Java, Scala, Python etc., every variable
stores the address of the object(value) which will be laid out in heap memory, and they
are called reference variables.

## Definition of a pointer

A pointer is a variable which can store the address of a variable or value.

*Examples*
```rust
let x: u8 = 1234521; // requires 1 byte(8 bits) to store "x" in memory
let y: &u128 = &x; // In a 64 bit architecture this will require 64 bits to store "y" in memory
let z: &&u128 = &y; // same bits as y

struct Point {
    x: i128,
    y: i128,
}
// It requires 256 bits to store a value of type Point in memory
let p = Point {
    x: -134,
    y: 1341,
}
let q = &p; // requires only 64 bits to store in memory of 64 bit architecture
```

In short every pointer variable of arbitrary types requires only 64 bits to be stored
in 64 bit architecture.

In computer memory every byte has a unique address. The address of a variable refers
to the first byte of the value of that variable in memory.
We can observe that these addresses are a runtime construct. When compilers compile our
program it generates binary code which will be executed by the operating system later (we can
write bare metal Rust also, but we are not concerned about this here). So every time the program
is executed we will observe different memory address for the same variable.

## Why Pointers are important?

Pointers provide the semantics to handle data that we don't have yet.
Any useful computer program needs to interact with the user. But when we write the program
we won't have access to what the user might give as input.
It also allows us to write polymorphic code using Trait Objects, which we will see later.
[Read more](https://en.wikipedia.org/wiki/Virtual_method_table).
It also allows the programmer to share an object(value) between different functions, threads
etc., without copying them every time, which leads to performance benefits but comes with the
cost of race conditions, hard to read and manage state across various function calls and threads
et., But again with Rust's powerful type system it is easier to write safe, correct, performant code
using pointers.

## Rust specific clarifications

The pointers that we have discussed so far are called shared references, which are **Thin pointers**.
Because they are compiled in a way that they only store the address of a value.
There are other kinds of pointers like smart pointers, fat pointers etc., which we will cover later
in the course.
