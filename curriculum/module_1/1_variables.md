# Immutable by default

In Rust variables are immutable by default, contrary to other statically typed languages
like C++, Java etc., where variables are mutable by default.

*Examples of immutable variables*

```rust
let x: i32 = 6;
let s: &str = "abcd";
let s_string: String = String::from("abcd");
```

### But why?

Robust codebases that use Java, C++ etc., will demand that a variable which does not mutate
to be constant (final in Java, const keyword in C++). One can observe that the most variables
are constants.
This restriction is put in place because it eases the difficulty in reasoning about the code.
If you know that a variable doesn't changes it's value then you don't have to think about in
while tracking the state changes when the code gets executed.
It also prevents accidentally mutating a variable.
There are several other reasons as well but this is the most important one.

*Examples for readability*

Below are some examples that you can experiment with yourself on [Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=9a549f8738d6c49c4f6a204b6094cec5):

```rust
#[allow(unused_assignments)]
fn main() -> () {
    let num = 2;
    num = 3; // compiler won't allow this
    println!("{}", num); // num is not changed
}
```

```rust
#[allow(unused_assignments)]
fn main() -> () {
    let mut num = 2;
    num = 3; 
    println!("{}", num); // num is changed
}
```

These problems are profound when working with shared objects/variables as shown in the example below.
Feel free to experiment with this on [Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=cf7212ce7d3f4d146107f38cb8aca4c3)

```rust
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn mut_shared_point(p: &mut Point) {
    p.x += 50;
    p.y += 10;
}

fn main() -> () {
    let mut point = Point {
        x: 10,
        y: 10,
    };
    
    mut_shared_point(&mut point); // pass in a mutable reference to a point
    
    println!("{:?}", point);
}
```

In the above example the caller of the `mut_shared_point` function should be aware that the point whose reference is being passed to might change, so this affects how the programmer thinks about the lines of code which comes after that function call.

We haven't talked about what a *mutable reference* is exactly (as indicated by `&mut`), but for now it simply means that we're allowed to modify the `Point` struct instance that is passed into the `mut_shared_point` function as an argument.

If the point is not declared as mutable, compilation will fail. 
If the point is not passed in as a *mutable reference*, compilation will fail as well.

Play around on Rust Playground to get a better feel for this and see what happens when certain variables or arguments are not declared as mutable.