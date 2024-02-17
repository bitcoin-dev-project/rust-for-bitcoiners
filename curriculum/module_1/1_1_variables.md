# Immutable by default

In Rust variables are immutable by default, contrary to other statically typed languages
like C++, Java etc., where variables are mutable by default.

*Examples of immutable variables*

```rust
let x: i32 = 6;
let s: &str = "abcd";
let sString: String = String::from("abcd");
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

```rust
fn f(x: i32) -> i32 {
    // By looking at the function signature we know that the variable x is not
    // changed within the function
    return x + 1;
}

fn mut_f(mut x: i32) -> {
    // Now we know that variable x is "potentially" changed within this function
    // Rust compiler will generate a warning stating that x need not be mutable if
    // it was declared mutable but not being mutated.
    x += 1;
    return x;
}
```
