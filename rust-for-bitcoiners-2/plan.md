# Course plan

## Introduction

In this course, students will learn Rust programming language by implementing concepts used
in bitcoin and interacting with bitcoin software and related crates(libraries) in rust.

### Why Rust for bitcoin?

Rust is a static strongly typed Systems programming language giving programmers control of computer
resources like CPU, memory, threads etc., directly in a safe way.
It achieves safety with the help of an expressive type system using Generics and traits
and borrow checking.

It is a widely accepted belief in the bitcoin community that bitcoin and related software should
be efficient and consume less computing resources so that it can be run on relatively cheap hardware.

C, C++ etc., gives the ability to write efficient, less resource-intensive code but does not guard
the programmers from introducing memory vulnerabilities. This is why Rust is now the go-to choice
for implementing bitcoin-related software.

### Prerequisites for the course

This course expects the students to have prior programming knowledge and understanding of basic concepts
like variables, loops, functions, user defined types etc.,
Reading of Chapters 1 to 4 of [Rust book](https://doc.rust-lang.org/book/title-page.html), it's not required have thorough understanding,
but get your setup up and ready for coding in rust.
Understanding of concepts like stack and heap memory, how [malloc](https://medium.com/@rehamshipl666/understanding-memory-allocation-in-c-with-malloc-e87d32134f1b), every
programming language relies on some version of malloc to do memory allocation.
[Chapter 4](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html) of rust book also discusses about this topic.



## Course modules

### Module 1

Introduction to variables, loops, arrays, integers, functions and pointers.

**Assignment**: Ceasar Cypher

### Module 2

Method Syntax, Traits, BigUint using elliptic curve cryptography.
Demonstration of a naive hashing algorithm using xor.

**Assignment**: Implement your own hashing algorithm

### Module 3

Enums and pattern matching with Result and Linked List type.

**Assignment**: Build your own bitcoin

### Module 4

Introduction to `bitcoincorerpc` and `bitcoin` crates.

**Assignment**: Interacting with bitcoin node and getting used to working with Result and Option

### Module 5

Demonstration of crates like `sha256d`, `secp256k1`, `bitcoin` and  `serde` by creating a signed 
transaction, parsing transaction data from text files and verifying their signatures.

**Assignment**: Given a list of transactions mine a new block.

### Module 6

Memory allocation in depth by building a graph data structure using Rust.

**Assignment**: Bitcoin blockchain privacy analysis by linking transactions using graph data structures.

### Module 7

Introduction to parallel programming in Rust.

**Assignment**: TODO

### Module 8

Introduction to asynchronous programming using `tokio`.

**Assignment**: Communicate with bitcoin nodes by sending valid messages, might include bip324 here.

## What is the outcome of the course?

This course is sponsored by Chaincode to get more programmers into Rust Bitcoin FOSS.
So we expect you to be a self-learner and come up with your own ideas. Every assignment is open ended
and can be extended as you wish. If you submit the assignment within the deadline your implementation
will be reviewed and you'll get timely feedback.

Three students from the first run of this course
have successfully contributed to [rust-coinselection](https://github.com/Bitshala-Incubator/rust-coinselect) repository so far.

## Regular session

During regular sessions groups will be formed.
Each group will be provided a set of questions related to rust and bitcoin.
Each student will be allocated a question and their job is to explain the concepts to others.
Moderators will be provided.

The initial 20 minutes introduction will be conducted by the instructor, followed by 20 minutes of group discussion
and the last 20 minutes will be reserved for clarifying open questions and issues.

## Office hours

This is a time specifically to address questions related to rust and bitcoin.
A form will be posted where you can add your questions in advance so that the instructor can be prepared.

## Suggested Reading

What Every Programmer should know about memory - [Ulrich Drepper](./Ulrich%20Drepper,%20Red%20Hat,%20Inc.%20-%20What%20Every%20Programmer%20Should%20Know%20About%20Memory%20(2007).pdf).
