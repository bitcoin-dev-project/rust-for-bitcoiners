# Why Current Rust resources are not enough?
The beginner friendly books like the [Rust Book](https://doc.rust-lang.org/book/)
does not prepare students mentally to use a complex programming language like Rust.

## Why is Rust complex?
Because it exposes the underlying OS, Hardware characteristics/features to the user.
Examples, 
- Rust int types expects the user to understand how the values are laid out in
memory.
- Rust compiler takes the ownership of deciding when to free up a heap memory (borrow checker),
so general programming habbits, idioms learnt by the user may hinder them to write valid Rust code.
- Thinking about safety from the very beginning (try implementing a counter), restricted mutability.
- Liftimes and thinking interms of scopes (Function frames in stack vs heap) [Compiler rules](https://doc.rust-lang.org/1.8.0/book/lifetimes.html)
- Traits and Generic programming are complicated in Rust because of Zero cost abstraction.
- Trait objects expects the user to know about dynamic dispatch, unlike Java where every 
method call is a vitual dispatch. So one should be able to reason about what can be known
at compile time vs things that can only be known at runtime. &T
- Working with datastructures are not intuitive because of mutable and immutable references.
- Dealing with OS threads directly and async runtime instead of green threads!
- Knowing the difference between OS thread sync primitives and async thread sync primitives.
- Hard to read generic traits and function signatures.
- Rust iterators, ownership, lifetimes etc., makes it harder to use for DSA style questions

The above reasons makes it harder to read docs and source files also.

##
1. Intro to Computer Arch
1. Intro to OS
1. Intro to Compilers

Some concepts from the above three can be included in regular course, with examples from C

Other than that we can follow the regular outline of the Rust book.

Learning Bitcoin, and teaching Rust and bitcoin job

Learning Rust for Bitcoiners