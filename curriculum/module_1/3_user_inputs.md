# User Interaction in Rust through command line

How to get input from user through commandline in rust?

```rust
use std::io; // std crate has io module

let mut input = String::new(); // has to be mutable because we have to fill this with user input
io::stdin().read_line(&mut input).expect("Failed to read line");
```
Refresh pointers concepts from [here](1_3_pointers.md).
The idea of the above code is that ``` stdin() ``` function will return a handle which will listen
for user input from the command line.
If the input is successfully read then the input variable to refer to that string.

