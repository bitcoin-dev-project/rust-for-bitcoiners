# Developing Software for bitcoin

What does it mean to develop software for Bitcoin?
Let's dial back and think about how Bitcoin software works.

In simplified terms, a Bitcoin node
* Crafts a message and broadcast it to it's peers
* Stores some messages permanently in it's file storage

Eventhough the above two points does not cover all the capabilities and features
of a bitcoin node it emphasizes the fact that in a distributed system creation,
reception and transmission of messages are the fundamental problems.

### Note

Bicoin node can be written in any programming language. A node is simply a
software running in a machine which follows bitcoin consensus rules.
Although obvious, I believe stating this, paints a better picture of the problems
that we are trying to solve.

## How do computers communicate?

Like humans, computers need to agree on the semantics of communications which are called
[communication protocols](https://en.wikipedia.org/wiki/Communication_protocol).
These communication protocols are language agnostic, meaning the messages which are being
transmitted should follow a well defined set of rules according to that particular protocol.
In a physical sense a message is just a string of bytes and every programming language has
support for arrays.

## Why Rust is good for communications?

By now you are familiar with rust compiler and would've noted that Rust is very specific
with types. Type safety helps in ensuring that only syntactically valid messages can be
created. With the help of `Result` type we will be able to handle the cases where the message
creation fails. A software which is written to hanlde all possible edge cases are called *Robust*.
Rust helps in creating Robust software.
