# How to represent numbers in Rust?

Modern computers (CPU level) support two types of numbers. Bounded integers and Floating point
numbers.
Programming languages like C and Rust supports both signed(Integers that we learned in Math) and
unsigned(Natural numbers) integers.

*Integer examples*
```rust
let x: i32 = 6
let y = 65 as u8;
```
The integer variants signed and unsigned are available from 8, 16, ..., 128 in rust.
**u32** means it uses 32 bits to store the unsigned integer. It can represent 2^32 numbers from
0 to 2^32 - 1.
```rust
let z: u8 = 256; // invalid will be a compile time error
// because Rust compiler knows that the maximum number a u8 can support is 255
```

*Floating point examples*
```rust
let x: f32 = 5.0; // valid
let y: f64 = 5; // invalid
let y: f64 = 5 as f64; // valid
```

There are only 2 variants of floating point numbers are supported in rust f32 and f64.
You can read more about them [here](https://en.wikipedia.org/wiki/Floating-point_arithmetic).
This crazy [Javascript example](https://stackoverflow.com/questions/23175363/javascript-parseint-behavior-when-passing-in-a-float-number) with parseInt function can be a food for thought as well.

## Type safety with numbers

In Rust we cannot mix numbers of different types together in a single expression.

**Examples**
```rust
let a: i32 = 5;
let b: f64 = 5.0;
let y = a + b; // Invalid
let z_f64: f64 = a as f64 + b; // Valid, you are explicitly stating that the variable a should
// be converted to type f64, implicit conversions like you would observe in C cannot be done here.
let z_i32: i32 = a + b as i32; // Valid

// Be aware of the conversion rules to avoid surprises.
```

## Copy trait

Numbers are copyable in Rust, meaning we can do the following.

```rust
let x: i32 = 5;
let y = x; // Value stored in x is copied to y
let z = x; // This is valid

// The value 5 exists in 3 different memory locations, because it gets copied in line 54 and 55.
```

Another example shows that we can pass `x` as an argument to a function and continue to use `x` in our current function.
[Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=4d1e9dd607a375b5065563a84bf4b3a7)
```rust
fn add_5(num: i32) -> i32 { // receives a copy of x as the argument
    num + 5
}

#[allow(unused_variables)]
fn main() -> () {
    let x: i32 = 5;
    let y = add_5(x);
    
    println!("{}", x); // prints 5
}
```

We won't be able to do this for non-copyable types, like a **String**.
There are different semantics because of the ownership rule.
Take a look at the example below.
[Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=cf786ab3bacd43e260e73ae5efa49d50)
```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(s1);

    println!("The length of '{}' is {}.", s1, len); // this will create an error
    // we're attempting to use s1 after it has been "moved" into calculate_length
}

fn calculate_length(s: String) -> usize {
    s.len()
}
```

## Why does Rust *move* variables?

Any type that *owns* data on the heap is typically *moved*.
In this case, a *String* is actually a *smart pointer* to heap data which stores UTF-8 valid bytes.
So when the pointer is passed as an argument, ownership of the heap data is moved from the variable in `main` to the variable `s` in `calculate_length`.
A new copy of the pointer is being made in `calculate_length` and then ownership of the heap data is given to that new pointer.
In Rust, only one variable can have ownership of heap data at any given time.
This concept of a single owner is how Rust efficiently manages memory.
Whenever that owner goes out of scope, Rust knows that it can safely clear the data owned in memory as well.

## Why are some types Copyable?

For numbers we know exactly how many bits are required to store them.
Can the same be said about Strings? The length of a String is unbounded, essentially
it can be any large number only bounded by the amount of RAM of the computer.
So if a length of a String is 1,000,000 then it effectively requires 8,000,000 bits plus
some additional bits to store them in memory. Is it efficient to copy them in every assignment and
function call?

In Rust we can create our own type. So if we can deterministically say how many bits will be required
to store them in memory then we can implement Copy trait for that type, else it should obey the move
semantics.

The rule is *If all the members of the struct are copyable then the struct can be copyable*.
