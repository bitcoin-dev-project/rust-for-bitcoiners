# Arrays and Slices in rust

## Arrays in Rust

An array is a fixed-size collection of elements of the same type. In Rust, arrays are defined using square brackets `[]`, and their length is known at compile time.

### Creating Arrays

Here are some examples of how to create arrays in Rust:

```rust
fn main() {
    let numbers: [u8; 5] = [1, 2, 3, 4, 5]; // An array of 5 integers
    let zeros: [u8; 4] = [0; 4]; // An array of 4 integers, all initialized to 0

    println!("{:?}", numbers); // Output: [1, 2, 3, 4, 5]
    println!("{:?}", zeros);   // Output: [0, 0, 0, 0]

    println!("{}", numbers.len()); // Output: 5
    println!("{}", zeros.len()); // Output: 4
}
```

As you can observe from the above code snippet `[u8; 5]` and `[u8; 4]` are two distinct
types in rust.

Since the size of the arrays are known at compile time they are always stored in the process
stack memory. If you want to store the array in heap memory then `Vec` is the go to type in most
cases.

## Slices in Rust

Slice is a fat pointer with type `&[T]`, where `T` represents any arbitrary type.
It is called a fat pointer because it contains the starting address of the array of memory and
it's length. One can have a mutable slice `&mut [T]` as well.

## Creating Slices

Slices are not standalone and are often inferred from structures like array, Vec etc.,

```rust
fn main() {
    // Slicing an array
    let array = [1, 2, 3, 4, 5];
    let slice = &array[1..4]; // A slice of array from index 1 to 3
    println!("{:?}", slice); // Output: [2, 3, 4]

    // Slicing a Vec
    let vector = vec![1, 2, 3, 4, 5];
    let slice = &vector[1..4]; // A slice of vector from index 1 to 3
    println!("{:?}", slice); // Output: [2, 3, 4]

    // Slicing a String
    let string = String::from("Hello, world!");
    let slice = &string[0..5]; // A slice of string from index 0 to 4
    println!("{}", slice); // Output: Hello
}
```

Slices behave like arrays, because under the hood they are just a reference
to an array, array enclosed in a Vec or a String etc.,

Looking up for an element or mutating an element at an index of a slice/array/vec
works very similar to how these structures are used in common programming languages
like c, java, javascript, python etc.,
