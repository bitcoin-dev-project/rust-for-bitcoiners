# Deref Coercion

For this topic of discussion we are interested in `Deref` and `DerefMut` traits for the
smart pointer types `Box` and `Rc`.

## Box

`Box` implements both `Deref` and `DerefMut` because Box only have a single pointer to the
value in the heap.
Specifically `Box<T>` implements `Deref<Target=T>` and `DerefMut<Target=T>`. which means
`&Box<T>`, `&mut Box<T>` can be coerced into `&T` and `&mut T`.

```rust
let mut a = Box::new(65 as i32);
let b: &i32 = &a;
let c: &mut i32 = &mut a;
```

the above snippet is a valid Rust code.

More examples,

```rust
fn deref_coersion(v: &String) {
    // Do something
}

fn derefmut_coersion(v: &mut String) {
    // Do something
}

fn deref(v: &Box<String>) {
    // Do Something
}

fn derefmut(v: &mut Box<String>) {
    // Do Something
}

fn deref_demo() {
    let ex: String = "example".to_string();
    let a: Box<String> = Box::new(ex);
    deref_coersion(&a);
    deref(&a);

    let mut ex: String = "example".to_string();
    let mut a: Box<String> = Box::new(ex);
    derefmut_coersion(&mut a);
    derefmut(&mut a);
}
```

The same arguments holds for `Rc` type also but we can't get mutable reference to the value in the
heap because it allows multiple references to that value, which means by the borrow checker rule
that value cannot have a mutable reference.

## Performance implications

Like any other structs `Box<T>` and `Rc<T>` values are stored in stack which has a pointer variable
which points to the value of type `T` stored in the heap.

If `Box<T>` or `Rc<T>` dereferences to the address of it's struct in the stack then it means
in order to get to the value in heap it has one additional indirection, that is we have to reach the
structure in the stack first and then using the pointer of the structure go to the value in heap.

By making the deref of these smart pointers to directly give the reference of the value stored in the
heap rust programs achieves a much better performance with one less indirection.
