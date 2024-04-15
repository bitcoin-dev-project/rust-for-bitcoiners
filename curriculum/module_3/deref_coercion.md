# Deref Coercion

For this topic of discussion we are interested in `Deref` and `DerefMut` traits for the
smart pointer types `Box` and `Rc`.

## Box

`Box` implements both `Deref` and `DerefMut` because Box only have a single pointer to the
value in the heap.
Specifically `Box<T>` implements `Deref<Target=T>` and `DerefMut<Target=T>`. which means

```rust
let mut a = Box::new(65 as i32);
let b: &i32 = &a;
let c: &mut i32 = &mut a;
```

the above snippet is a valid Rust code.

What also means is that let's say we are working with a collection for example `HashSet<Box<T>>`,
which has the following method,

```rust
pub fn contains<T>(&self, value: &T) -> bool
```
the contains method is actually looking for `&T` and not `&Box<T>` because it automatically coerces
to the former.

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
