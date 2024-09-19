# Bitcoin scripting Arithmetic using Rust

Scripting is the programming language used in bitcoin to validate the ownership of bitcoins and validation of a transaction.

Using scripts we define the contract which has to be fulfilled.

The contract is a program, if the correct inputs are provided it will evaluate to true.

For this assignment we will be using Bitcoin scripting semantics to evaluate integer arithmetic expressions.
This I believe is a better way to teach about scripting rather than directly jumping into complicated payment contracts.
Because *everyone* understands addition and subtraction 😉.	

## What is the syntax of Bitcoin scripting?

It is precise to think of bitcoin scripting language as an assembly language.
It has a set of commands which has a corresponding opcode(a byte).
The input data are part of the Script as well. A script can have any other arbitrary data as well.

Review [this](https://en.bitcoin.it/wiki/Script) document for thorough overview of script op codes.

## How to represent Scripts in Rust?

We need a type which identifies various op codes, there are `PUSH` opcodes to include arbitrary fixed sized data in scripts.
Scripts are evaluated word by word using a stack to store intermediate results.

For example consider this simple script `[OP_1, OPP_1ADD, OPP_1ADD]`.
While reading the first opcode, number 1 is pushed into the stack `[1]`.
The second opcode increments 1 to the top element of the stack resulting in `[2]`, similarly after the last one we have `[3]` as the stack state.

It is this simple, actually Assembly language is also very simplistic in design and easy to understand. The problem is that it is very tedious to write complicated logic with this simple design.

But as a programmer it is very easy to write a function to evaluate the scripts and that is exactly what you are going to do in this assignment.

## Assignment task

Complete the functions and types as specified in the `main.rs` file.
The goal is to pass all the tests. You are expected to handle opcodes related to *Arithmetic*, *FLOW CONTROL*, *CONSTANTS* and *STACK* only.
Not all of them are necessary.

**BONUS**: You are free to include logic to evaluate additional opcodes and
complicated scripts. But make sure to test them too!
