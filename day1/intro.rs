#[warn(unused_variables)]
/**
 * Move semantics and Copy semantics
 * If a type does not support Copy then it is moved on `=`
 * When will the Copy derivation succeeds?
 */
#[derive(Clone, Copy)]
struct Index(i32); // Zero cost abstraction wrapper, the machine code gerated will be similar to that of i32
// It's only purpose to check the semantic correctness of the type

fn add1(short: i16) -> Option<i16> {
    short.checked_add(1)
}

fn main () {
    let i = Index(6);
    let j = i;
    println!("{}", i.0);

    let short: i16 = 32767;
    let short1 = add1(short);
    println!("{short1:?}");
}
// int x = 5;
// x = 5;
// var x = 5;, let x = 5; 

// Is Rust thread safe? https://github.com/rust-lang/rust/issues/26215
