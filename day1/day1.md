# What this course is about?
The motivation for doing this is to convince you why Rust is awesome.
Though Rust is ranked as the most loved programming language year after year,
it was not adopted and it's rise was not impressive compared to the most hated
and memed languages like JS and Python.
The reason is that Rust was not designed to be used by beginners.
There are pleathora of programming courses/projects using JS and Python targetted towards beginners
and there are almost none in Rust (I know Michael Snoymen is teaching Rust to school kids).

## Why Rust is so hard for the beginners?
I'm framing the question this way, because dynamic programming languages like Python, Ruby, JS,
C (RIGHT!) etc., hides a lot of complexities from the users.
Rust exposes everything to the user and expects the user to be explicit about what they want.
Rust is very easy and convenient for experienced programmers because it prevents them from
shooting themselves in the foot unlike Python (It's just a wrapper over C).
Read this monstrosity to understand the context [instagram disables gc?](https://instagram-engineering.com/dismissing-python-garbage-collection-at-instagram-4dca40b29172).
It is very hard to run into such problems using Rust.
The point is Rust is a nuisance if you want to build a toy or a solo small scoped projects.
But it has very few competitors if you want to build highly secure, robust, performant, scalable,
blah, blah ... systems.
With C and C++ it's very hard to get the robust and secure part correct.
Now you understand why Rust is the most loved programming language in StackOverflow?
(Only the experienced nerds vote for their favourite language? I don't remember casting a vote ever).

# Day 1 plan
Rust is a systems programming language originally created to replace C/C++.

## Why replace C?
C gives us low level control over how memory is used to the programmer. Other Low level OOP languages
like Java, C#, go etc., use garbage collection to free up the allocated memory.
Fun fact: Go a garbage collected language might enable programmers to use memory efficiently in a
larger project than C++, because C++ supports inheritance(BAD PRACTICE baked into the language).
C expects the programmers to write memory safe code. Rust restricts the programmers so they can
only write memory safe code.
So I will encourage low level thinking first and help you understand the difference between C and Rust
in the beginning.
Later we may look into the Rust's type system which closely resembles what we see in Haskell 
(Hindley-Milner Typing), and async runtime, which allowed Rust to be used wide variety of applications.

## What is the plan for day1?
1) Understand Integer types and how they differ from C
2) Stack and Heap based memory and why as a Systems programmer you need to think about them
3) Intro to borrow checker

# References
Casey Muratori https://www.youtube.com/user/caseymuratori


# Day 1 summary

## How to define an int variable in Rust?
In dynamic programming languages like Python or JS defining an int variable is as easy as,
```
x = 5 # python
var x = 5 # JS, we don't have integer just NUMBER??
```
In statically typed languages like C or Java we do something like,
```
int x = 5; // C & Java
```
But defining an int variable in Rust a programmer suddenly have so many choices.
i8, i16, i32, i64 and i128 which are supported by the compiler by default.
Here the numbers after i denotes the number of bits required to store that int variable.
This is something that people usually complain about in Rust, which is there are too
many options and it might be confusing if you are not sure what exactly is that thing you
want in the first place.

## Rust compilers effort to make defensive coding easy

For example in C we can use "short" variables which are nothing but i16 in Rust.
Maximum integer that can be represented in 8 bits is 32767, binary format is (0111 111).
But if you write the following in C,

```
short x = 32768; // binary format is (1000 0000)
printf("%hi", x); // What will be the output?
```
Integers are represented in bitlevel using a format called 2's complement in more than
99% of the hardwares out there.
If the first bit starts with 0 then it is positive else negative.
So the value "32768" will be interpreted as a negative number. If you forgot the range
then you are in for a surprise.
Doing the same in Rust will give you a very readable compile time error.
[For Rust example](./intro.rs).
Rust compiler will throw overflow error during compile time if it detects any overflow
during [constant folding](https://en.wikipedia.org/wiki/Constant_folding).
For dynamic overflows it panics during runtime, (C will continue to run with -ve values when 
you add 2 +ve numbers for example.)

## Stack vs Heap memory

When a process is started by the OS it will get a fixed array of memory called *Stack* memory,
and a dynamically growing memory called *Heap* based on the requirements during runtime.
Suggested reading to understand them better would be Computer Architecture and Design by Patterson
and Operating Systems Concepts by Silberschatz.
The point is allocations in Stack is vitually free (remember that it was already allocated when the
process is started) and automatically managed by the OS (Stack frame just moves up and down and the
values gets overwritten.)
Any programming language relies on System libraries provided by the OS to talk with OS.
So if you want to free some memory you have to call free(*pointer_to_mem), and it will seg fault
if you try to free the stack memory or the memory that does not belong your process.
But nevertheless in C/C++ one have to manually insert free, in garbage collected languages
a thread will fire *free* calls whenever it finds some *unreachable* addresses.

## Borrow Checker in Rust
Borrow checker has two semantics Copy or Move.
Every value in Rust has a Type and it is Strongly Typed (C/C++ are weakly typed meaning 
implicit conversions, which are not checked for correctness by the compiler).
If a type implements a Copy trait then whenver you do something like,
```
let x: i32 = 50; // any primitive type is copyable
f(x); // some function call
let y = x; // reassignment works
printlin!("{x}"); // works
```
because in each step the value is copied. Why these values are copyable? Because it is easier
this way, in 64 bit architecture the size of the pointer itself is 64 bits!

Let's look into Move semantics.
```
struct Index(i32); // Type does not implement Copy, but still the compiler stores the values in Stack

let x = Index(56);
f(x);
let y = x; // ERROR, because the value x is moved to function *f*
```
Rust does not insert free calls on the variable x here, because it is stored in Stack.

### When does Rust decides to store a value in Stack?
If it can determine the size of the variable at compile time.

We will discuss more about pointers and references in the future classes.