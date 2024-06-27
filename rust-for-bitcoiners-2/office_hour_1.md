### Similarity between Rust and C++

Explicit control over Control usage of CPU and memory allocations.


**Example**

How the size of the struct is determined?

In [chapter 2](https://learning.oreilly.com/library/view/rust-for-rustaceans/9781098129828/c02.xhtml#h1-123456c01-0001) of  Rust for Rustaceans book Jon Gjengset gives a detailed explanation.

### Why Rust is special?

Memory Safety and it's type system.

### OOP is Fiat Standard programming paradigm

OOP standard <- Fiat Standard
Design patterns, Best practices, ... collection of opinions, dogma and status quo.
They will focus on code reuse which is not a well defined objective and often results
in uninteded consequences.

### rust and functional programming languages are Bitcoin Standard

Functional programming languages they are designed based on clear mathematical laws.
Rust ensures memory safety and tries to follow the type system of Haskell in a pragmatic
way without comprimising on performance and efficiency.

[Roc](https://www.roc-lang.org/) pure functional programming language tries to be as fast as
Rust, still in early stages of development.

### Why &str has to be converted to &[u8] in the assignment 1

Type conversions should be explicit in Rust.
str is actually a [u8] (array of u8) internally, and `as_bytes` function gives the
copy of that array.

### What is Race condition?

Explained in the meeting, refer this excelled wikipedia [article](https://en.wikipedia.org/wiki/Race_condition).

### How Rust avoids race condition?

With the following rule,

Multiple read only borrows vs single write only borrow
Multiple immutable borrows vs single mutable borrow

### Single Owner rule:

**Owner**:

The variable of a value is called the owner of the value.

**Rule**:

Any value should have a single Onwer, it allows the compiler to decide when to throw
away that value which is stored in the memory.

Helps the rust compiler to insert the `free` in rust terms `drop` sys call in appropriate places,
Basically whenever the variable goes out of scope.

### Take home question:

Multiple immutable borrows vs single mutable borrow rule is required only in multi threaded programs.
This Rule prevents data races, which can happen only in multi threaded code.
Why Rust enforces this rule for single threaded code also?
