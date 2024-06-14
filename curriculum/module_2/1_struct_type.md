# Data Types

## Structs

In Rust structs are types that are composed of other types.
You can think of structs as product types similar to a Cartesian product between sets.
It is a collection of *n* types. At any point in time the value of a struct is the combination of all the *n* types.
This is very similar to a simple class in Java or struct in C.
In software design terms, `struct` enables Composition and record types.

**Example 1**

In rust the supported primitive unsigned integer types are `u8`, `u16`, ..., `u128`.
In order to do elliptic curve cryptography we need to be able to represent unsigned
integers using 256 bits,

```rust
// capital `U` is used to signify that this is a user defined type.

struct U256 {
    x0: u128, // least significant bits
    x1: u128, // most significant bits
}
```

The compiler knows that in order to store the value of type `U256` it would require 256 bits because of the field types.
Hence every value of `U256` will be stored in the stack since its size is known at compile time.
Moreover rust requires that the value of every field type should have a known size at compile time.

**Example 2**

If that is the case, then how do we deal with dynamically-sized data that cannot be known at compile time?
Let's look at an example of a `vec` which is often modified at runtime. 
Here is a simplified version of how `Vec` is represented in rust,

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

The `RawVec` type has a pointer field, `ptr`, which points to a block of memory (aka **buffer**) that is in the heap.
The `alloc` field contains a type that has methods to manage heap memory allocations.
The `cap` field indicates the buffer's capacity or allocated space in memory.

To better understand `cap` let's consider a scenario,
```rust
let xs: Vec<u32> = Vec::new();

for i in 0..10 {
    xs.push(i);
}
```

Initially `cap == 0`. When the first data push happens `cap` will be increased to `4`.
Therefore, during the 2nd, 3rd and the 4th push the capacity will not be changed, and no
new allocation will happen. During the 5th push, however, a new buffer that will hold 8 `u32`s will
be created and the contents from the current buffer will be copied to the new buffer and
`cap == 8`. After this whenever the `len == cap` a new buffer of `2 * cap` will be created
and so on ..

This process is known as *reallocation* and is quite costly in terms of performance. 
If you know ahead of time that you will be pushing a lot of data to a `vec`, it's good practice to invoke a `vec` using the `with_capacity` method to indicate how much space to allocate ahead of time and avoid costly reallocations.

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

The purpose is encapsulation, `String` can't be used like a `Vec<u8>`. Checkout the documentation
for [String](https://doc.rust-lang.org/std/string/struct.String.html) and [Vec](https://doc.rust-lang.org/std/vec/struct.Vec.html), for more info.
