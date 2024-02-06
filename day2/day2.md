# Lifetimes in Rust

Liftime based on scope of a variable, but it is a compile time construct,
so that when the lifetime ends the borrow checker will destroy the variable.

In rust when a variable is assigned a value, it is said to own the value.
That value can be borrowed by another variable, which is called the borrower.

Rust compiler has to make sure that the lender outlives the borrower.

Every value has a liftime type usually inferred by the compiler.
Most of the times explicity lifetime annotations are not necessary.

## References to a value in heap

In rust one cannot directly return a reference to a heap allocated value
like in [c](returning_reference_to_heap.c). Once has to use one of the suitable
[allocators](https://doc.rust-lang.org/alloc/).
So if a function explicitly returns a reference then it should be from one of
the references that it gets as an input, when it has more than one reference
in the input, then we have to explicitly specify the type annotations as shown [here](returing_references.rs).
Without explicit annotation we will get this [error](explicit_liftime_required_error.png).