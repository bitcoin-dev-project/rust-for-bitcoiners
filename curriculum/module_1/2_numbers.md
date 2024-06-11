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
For non copyable types like **String** we have different semantics because of the ownership rule.

```rust
let s = String::from("abcd");
let s1 = s;
println!("{s}"); // ERROR: borrow of moved value: `s`
// Because String is not copyable, hence move semantics apply
// Because the value owned by s is moved to the variable s1
// s1 owns the value which was once held by s
// Now variable s don't own or refer to any value
// Essentially the ownership of the value is getting moved from one variable to another.

// The value `String::from("abcd")` exists in a single point in memory, and it is not moved.
```

## Why some types are Copyable but others are not?

For numbers we know exactly how many bits are required to store them.
Can the same be said about Strings? The length of a String is unbounded, essentailly
it can be any large number only bounded by the amount of RAM of the computer.
So if a length of a String is 1,000,000 then it effectively requires 8,000,000 bits plus
some additional bits to store them in memory. Is it efficient to copy them in every assignment and
function call?

In Rust we can create our own type. So if we can deterministically say how many bits will be required
to store them in memory then we can implement Copy trait for that type, else it should obey the move
semantics.

The rule is *If all the members of the struct are copyable then the struct can be copyable*.
