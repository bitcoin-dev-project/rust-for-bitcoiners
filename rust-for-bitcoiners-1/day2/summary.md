# Summary of 2nd Lecture

While reading this summary find the code examples [here](demo/src/main.rs).

## Enums and pattern matching

Enums are called sum types while structs are called product types, because a value
of an enum can be one of it's variants while the value of a struct should have all the fields
filled up which is essentially a cartesian product (discrete maths!).

A enum of the form
```rust
enum Expr {
    Integer(i32),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
}
```
has type name `Expr` and three constructors `Integer`, `Add` and `Sub`.
So a value of type `Expr` can constructed using one of the above 3 variants and hence can be destructured
using one of those three variants.

How enums are laid out in memory can be found [here](https://doc.rust-lang.org/reference/items/enumerations.html).

In Rust `Box` is used to allocate a block of memory in the heap. One simply has to do something
like `let reference = Box::new(some_value)`. Here the value held by the variable `reference` is very
similar to what we see in a typical OOP language.
Read more about stack vs heap in Chapter 4 of Rust book [here](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html).

### Why Box is required to define recursive types?

As you can observe `Expr` is a recursive type as `Add` and `Sub` takes arguments of type `Box<Expr>`.
But why `Box` is needed? Rust like C++ will try to allocate values as much in the Stack as possible,
because it is much faster and cheaper to maintain (OS does all the work!).
In order to allocate a value in the Stack Rust needs to know the size of the type at compile time.
There are sizes of types which can't be known at compile time like
dynamically allocated array (Vec in Rust), HashMap etc.,

But the pointer to those objects is typically the address of the first byte which is nothing but
[usize](https://doc.rust-lang.org/std/primitive.usize.html) in Rust.

The result of the `Box` is just a pointer and hence the size of `Box<Expr>` is the size of `usize`,
which is typically 32 or 64 bits based on the CPU architecture.

### Pattern Matching on Enums

The matching for the `Expr` enum is demonstrated below,
Note that the destructured value will transfer the ownership of the data to the associated
variable names in the destructors.

```rust
fn evaluate_expr(expr: Expr) -> i32 {
    match expr {
        Expr::Integer(value) => value,
        Expr::Add(e1, e2) => evaluate_expr(*e1) + evaluate_expr(*e2),
        Expr::Sub(e1, e2) => evaluate_expr(*e1) - evaluate_expr(*e2),
    }
}
```
The `*` operator which is called dereferencing or unboxing in Rust allows us to get the data
pointed to by the reference, remember `Box` is just a reference.

## Method call syntax

Using the impl keyword we can associate functions and methods to a type.
Examples,
```rust
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    // Associated function
    fn new(x: i32, y: i32) -> Point {
        Point {x, y}
    }

    // Associated method which takes the ownership of the Point value
    fn lost(self) {
        println!("point {:?} will be destroyed when this function terminates", self);
    }

    // Associated method which takes the mutable reference of the Point value
    fn set_x(&mut self, new_x: i32) {
        self.x = new_x;
    }

    // Associated method which takes the read only reference of the Point value
    fn get_y(&self) -> i32 {
        self.y
    } 
}
```

## Traits

Traits provides a mechanism to unify different types which behaves in a certain way,
that is according to the trait defintion.
Examples,
```rust
struct Circle {
    center: Point,
    radius: u32,
}

struct Square {
    center: Point,
    side: u32,
}

trait Shape {
    fn area(&self) -> u32;
    fn perimeter(&self) -> u32;
}

impl Shape for Square {
    fn area(&self) -> u32 {
        self.side * self.side
    }
    fn perimeter(&self) -> u32 {
        self.side * 4
    }
}

impl Shape for Circle {
    // Let the value of pi be 3 for illustration 
    fn area(&self) -> u32 {
        3 * self.radius * self.radius
    }

    fn perimeter(&self) -> u32 {
        2 * 3 * self.radius
    }
}
```

Now we have two types `Square` and `Circle` which are distinct in nature but both of them
implements the `Shape` trait, i.e they share a common behaviour.

Now any function that expects a value of type `Shape` as input can be used by the both of the types.

Then we discussed how to implement a simple enough Singly Linked List in Rust and implemented the
Iterator Trait for that `List` so that it can use all the functions defined for the Iterator trait.
Refer the [source code](demo/src/main.rs) where I have written extensive comments.

Read more about Linked List in Rust [here](https://rust-unofficial.github.io/too-many-lists/).
Completing this will give you thorough understanding of memory allocations in Rust.
