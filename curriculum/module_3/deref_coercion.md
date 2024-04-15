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
