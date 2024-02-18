# Can you see why Rust is best fit for Bitcoin related applications?

1. Rust makes invalid data hard to represent, it either detects overflows(numerical and buffer)
at compile time or panics and throws an error if it is encountered in runtime. This prevents
invalid data being stored in blockchain (remember blockchain is immutable).

1. It gives control over memory resources to you the programmer, and helps you write efficient code.
This might be crucial because once bitcoin is adopted widely businesses might need their own servers
to process customer transactions. With Rust the cost of running a server is cheaper.

1. Rust have Generics and Traits which are found in high level programming languages like Haskell, Scala
etc., This allows you to write expressive reusable code without sacrifising type safety.
