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
```

## Copy trait

Numbers are copyable in Rust, meaning we can do the following.

```rust
let x: i32 = 5;
let y = x; // Value stored in x is copied to y
let z = x + 4; // The is valid
```
For non copyable types like **String** we have different semantics because of the ownership rule.

```rust
let s = String::from("abcd");
let s1 = s;
println!("{s}"); // Invalid, because variable s don't have own or refer to any value
// Because String is not copyable
// Because the ownership of the value `String::from("abcd")` is moved from s to s1
```
