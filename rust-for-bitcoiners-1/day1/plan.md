# Day1 plan

"Contraints liberates"

1. Introductions
1. Why Rust?
    1. Memory Safety
    1. Modern Type System
    1. Compiler does a lot of work to get it right
    1. Borrow Checker for low memory footprint, cons will be only a subset of correct code can be written in safe rust, that's why Rust has unsafe blocks.
1. What does Rust mean for bitcoin ecosystem?
1. variables
    1. Why immutable by default
    1. Idea of ownership
    1. Move mechanism
1. numbers
    1. Representation
    1. Emphasis on correctness
1. pointers
    1. Demonstrate swapping of two variables
1. conclusion

"Assuming that you can avoid undefined behavior in C and C++ is like assuming you can win a game of chess simply because you know the rules" - Programming Rust book


Why C?
GC golang, Java, Python, JS, Haskell we have performance penalty.

free(pointer to memory location);

A pointer in C is just the address of a byte in your RAM.