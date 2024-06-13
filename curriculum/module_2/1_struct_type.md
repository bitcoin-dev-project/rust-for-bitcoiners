# Data Types

## Structs

In Rust structs are product types, think about Cartesian product between sets.
It is a collection of *n* types. At any point in time the value of a struct is the
combination of all the *n* types.
This is very similar to a simple class in Java or struct in C.
In software design terms, `struct` enables Composition and record types.

**Example 1**

In rust the supported primitive unisigned integer types are `u8`, `u16`, ..., `u128`.
In order to do elliptic curve cryptography we need to be able to represent unsigned
integers using 256 bits,

```rust
// capital `U` is used to signify that this is a user defined type.

struct U256 {
    x0: u128, // least significant bits
    x1: u128, // most significant bits
}
```

The compiler knows that in order to store the value of type `U256` it would require 256 bits.
Hence every value of `U256` will be stored in the stack.
Moreover rust required that the value of every type should be know at compile time.

**Example 2**

If so then how dynamic data who's size that can only be known at run-time can be represented?
Simplified version of how `Vec` is represented in rust,

```rust
struct Vec<T, A: Allocator> {
    buf: RawVec<T, A>,
    len: usize,
}

struct RawVec<T, A: Allocator> {
    ptr: Unique<T>, // Size of a pointer is either 32 or 64 bits based on the CPU architecture
    cap: Cap, // just a wrapper over `usize`
    alloc: A,
}
```

The `RawVec` type has a pointer to a block of memory (aka **buffer**) which is in the heap.
`alloc` field has the methods to manage heap memory allocations.
To understand `cap` field let's consider a scenario,

```rust
let xs: Vec<u32> = Vec::new();

for i in 0..10 {
    xs.push(i);
}
```

Initially `cap == 0`, when the first push happens `cap` will be increased to `1 == 4`,
So during the 2nd, 3rd and the 4th push the capacity will not be increased, meaning no
new allocation would happen. During the 5th push the a new buffer to hold 8 `u32`s will
be created, the contents from the current buffer will be copied to the new buffer and
`cap == 8`. After this whenever the `len == cap` a new buffer of `2 * cap` will be created
and so on ..

NOTE 1: The `RawVec` type uses unsafe code to handle allocations, and pointer management stuff.
NOTE 2: The size of both `Vec` and `RawVec` to be stored in stack of the process is known at compile-time.

**Example 3**

Let's look at a widely used wrapper type,

```rust
struct String {
    vec: Vec<u8>,
}
```

These wrapper types are zero-cost abstractions, meaning these exists only at run-time.
The number of bytes required to store a value of `String` is equal to the number of bytes
required to store the underlying `Vec<u8>`.

The purpose is encapsulation, `String` can't be used like a `Vec<u8>`. Cehckout the documentations
of [String](https://doc.rust-lang.org/std/string/struct.String.html) and [Vec](https://doc.rust-lang.org/std/vec/struct.Vec.html), for more info.
