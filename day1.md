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
Read this monstrosity to understand the context [https://instagram-engineering.com/dismissing-python-garbage-collection-at-instagram-4dca40b29172](instagram_disables_gc?).
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
