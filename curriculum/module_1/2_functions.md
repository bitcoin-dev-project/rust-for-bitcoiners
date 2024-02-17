# Function

Functions are the fundamental abstraction in Rust. In fact functions as an abstraction
is enough to represent any Turing Machine. [Church Encoding](https://en.wikipedia.org/wiki/Church_encoding), [Church Turing Thesis](https://en.wikipedia.org/wiki/Church%E2%80%93Turing_thesis).

## What is a function?

In mathematical terms a function is a mapping from set A to set B. In programming we can think
of a type as a set.
So a Rust function takes as input an **n** number of types and gives a corresponding output which
is also represented using a type.
Rust functions are not like a pure mathematical function, but it's lot simpler to reason and write
code with mathematical framework in mind.

*Examples*
```rust
fn inc(x: i32) -> i32 {
    x += 1
}

fn mul(a: u64, b: u64) -> u64 {
    a * b
}
```

### How to use functions?

Code sinppets to use the functions are given below,

```rust
let a: i32 = 6;
let b = inc(a); // valid because type of a and the function signature of inc matches
// The type of b is automatically inferred to be i32 from the return type of inc function
let c: i64 = inc(a); // invalid because the output type of inc is not compatible with i64.

let x: i64 = 50;
let y = inc(x); // Will be an error because
```

This might feel too restrictive. Essentially waht we are seeing is that inorder to define an
incrementing function for all numbers then we have to duplicate the same function code with
different function signatures.

In python for example we do this,
```python
def inc(x):
    return x + 1
```
The above function will work both on integers and floating point numbers. But
what will happen when we do ```inc("some string")```? we get a runtime error.
In Javascript we will get hard to predict, crazy outputs.

Rust ensures typesafety but this means extra work for programmers.
But Rust with the help of Generics and Trait constraints eases most of the problems in a safe way.

We can rewrite the inc function in Rust in the following way,

```rust
fn inc<T>(x: T) -> T
where
    T: std::ops::Add<Output = T> + From<u8>
{
    x + T::from(1u8)
}
```
The function signature says that the **inc** function will accept any input of type *T* if and only if
it satisfies the following constraints.
1. It should implemnent the Add trait in std::ops
1. It should implement the From<u8> trait

If a type implements Add trait means it can be used with "+" operator. This is similar to type classes in
Haskell.
We need to add 1 to type *T* so we require that we should be able to convert *1* from *u8* to that type.
1 requires just one bit to represent it, so the smallest possible unsigned integer is used.

These type constraints allows polymorphic code but at the same time it ensures type safety.
```rust
let s = "Aeda";
inc(s); // Invalid, will be a compile time error.
```

## Readability

In Software Engineering readability does not mean is it easy to read like English.
It means how easy it is to reason about the code just by looking at them.
If you are experienced in dynamic programming languages like Python, JS, Ruby etc., it might
sound crazy when I say that Rust is more readable than any dynamic programming language in most
cases.
Because just by looking at the type signature of a function you can guess what the function might do.
But since Rust doesn't have GC and forces the programmer to think about pointers, lifetimes etc.,
it adds to the cognitive load of a developer. But the advantage is that Rust programs are very efficient.
