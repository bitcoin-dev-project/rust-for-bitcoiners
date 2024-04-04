# What is Box?
In rust `Box` is a smart pointer that gives access to allocate values in heap.
It is a smart pointer because the compiler will automatically decide  when to
free that value in the heap.

```rust
let x: i32 = 5;
let box_x: i32 = Box::new(x); // allocates the value x in the heap
// box_x is an value of type Box(smart pointer) which has the address of the allocated value
```
Just like any other value in Rust the value of type `Box` also has a single owner,
in the above example `box_x` is the owner.
When the variable `box_x` goes out of scope the value allocated in the heap will be freed.
`Box` is not `Copy` so if you pass it as an argument to a function or assign it to a new variable
it will obey the `Move` semantics.

`Box` is part of the safe Rust meaning, it will always point to a valid memory location, it can
never point to a `NULL` and it works in a well defined manner unless the allocation in heap itself
fails because of no free memory in the RAM.

## If there is no NULL supported then how to represent missing values?

Here comes `Option` with `Box` to the rescue.

```rust
struct Person {
    name: String,
    age: u8,
}
fn may_exist() -> Option<Box<Record>> {
    // Do some computation
    if computation_succeeded() {
        Some(Box::new(Person{
            name: "name",
            age: 42,
        }))
    } else {
        None
    }
}
```

`None` variant of the `Option` can be used to denote missing values.
unwarpping a `None` results in `panic` which is still a well defined, reproducible behaviour.

## But Why do we need to allocate memory in Heap?

1. Recursive data types
1. You need to transfer the ownership of a value without copying it
1. 