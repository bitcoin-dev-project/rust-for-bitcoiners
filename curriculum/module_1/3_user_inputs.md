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

## Strings, what are they actually?

In computers everything is just a string/tape of bits(0 or 1). A turing machine is just an infinite
length tape. The types that we use in the programming language are compile time constructs.
So whenever your program receives an input it will be in the form of string.
It is programmer's responsibility to decode the string into an appropriate type.
And if a program wants to ouput something it has to encode that type to a proper string value.
We will see this pattern of encoding and decoding whenever the program interacts with outside world,
through command line, network calls using APIs etc.,

## How do we read an integer input in Rust?

The idea is to get the string input first and then try to decode it as *i32* for example.
Note that not all strings can be converted to a string, so we have to do proper error handling.
Rust compiler supports that with Option or Result types. In Pytho, Java for example we will have
runtime exceptions. In C `atoi` function will interpret the errors as 0, this is really bad because
0 is a valid number.

```rust
// let's try to convert the input string in the previous example to an i32
let x: Result<i32, _> = input.parse();
    match x {
        Ok(num) => println!("Parsed number: {}", num),
        Err(e) => println!("Failed to parse: {}", e),
    }
// This works because in Rust all the built in types have implemented `FromStr` trait
```

There are advanced libraries to parse the strings to appropriate types, we will discuss about them
later in the course.

## Getting commandline arguments in Rust

This is pretty straight forward and you can read it from [here](https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html).
