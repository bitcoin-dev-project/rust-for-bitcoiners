# Lifetimes in Rust

Liftime is based on scope of a variable, but it is a compile time construct,
so that when the lifetime ends the borrow checker will destroy the variable.

In rust when a variable is assigned a value, it is said to own the value.
That value can be borrowed by another variable, which is called the borrower.

Rust compiler has to make sure that the lender/owner outlives the borrower.

Every value has a liftime type usually inferred by the compiler.
Most of the times explicity lifetime annotations are not necessary.

## Why we may need explicit lifetime annotations in function signature?

In rust one cannot directly return a reference to a heap allocated value
like in [c](returning_reference_to_heap.c). One has to use one of the suitable
[allocators](https://doc.rust-lang.org/alloc/).
The bottom line is if a function accepts no reference as input it cannot return a reference in
output directly without using allocators in heap like Box, Rc and Arc.

So if a function explicitly returns a reference then it should be from one of
the references that it gets as an input.
If a function accepts only one input as reference and returns that reference(it could be mutable)
in output, then explicit lifetime declaration is not necessary.
When it has more than one reference in the input, then we have to explicitly specify
the type annotations as shown [here](returing_references.rs).
Without explicit annotation we will get this [error](explicit_liftime_required_error.png).

## Summary of day2 lecture

In low level language like C and Rust we have the ability to directly refer to a memory
location. A memory location when a program runs is nothing but a hexadecimal number.
In C we can arbitrarily refer to any such value, but if a process tries to access a memory
which is not allocated to it by OS, that will result in a SEG fault and the process will be terminated.
[Example in C](arbitrary_pointers.c). Sometimes by chance that arbitrary memory might belong to the
process and we might not get that SEG fault.

We don't want to deal with such undefined behaviours in Rust, so the compiler will refuse to compile
such code.

If a function f calls another function g then the lifetime of f is greater than that of g,
because only after g completes f will complete. In the same sense the variables defined in f are
said to live longer than variables defined in g. If x is a variable in g then f cannot have a
reference to x because after g completes x does not exist, but it's reference can exist in f.
The above unpredictable scenario is allowed to happen in C, [look at function g](returning_reference_to_heap.c).
These kind of topics are better discussed using pictures so please refer these links,
[one](https://icarus.cs.weber.edu/~dab/cs1410/textbook/6.Functions/scope.html), [two](call_stack.pdf).

In C a function can return a reference to any memory location. Ideally the function will return the
address of some memory location in heap, [look at function f](returning_reference_to_heap.c).
[Here](outer_outlives_inner.c) an example is given where a struct still holds a reference to an
illegal memory location, these scenario is called dangling pointers.

Lifetimes are closely tied to scopes in the call stack of a process, but it is a set of compiler time
rules in rust. Every variable in Rust have a type based on it's value and another type based on it's
lifetime(scope). Usually lifetime annotations are elided(omitted), but sometimes we need explicity
annotations.
Examples can be found in rust files.

See you in next class, keep learning!